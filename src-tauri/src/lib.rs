use std::{fs, path::PathBuf};

use anyhow::{Context, Result as AnyResult};
use serde::Serialize;
use tauri::{AppHandle, Manager};
use tauri_plugin_sql::{Migration, MigrationKind};
#[cfg(feature = "tauri-plugin-log")]
use tauri_plugin_log::Builder as LogPluginBuilder;

const DB_URL: &str = "sqlite:data/lokalbuku.db";
const DB_FILE_DESCRIPTION: &str = "lokalbuku primary schema";

#[derive(Clone, Copy)]
struct MigrationDef {
  description: &'static str,
  sql: &'static str,
}

const MIGRATION_DEFS: &[MigrationDef] = &[
  MigrationDef {
    description: "create_system_meta_table",
    sql: r#"
      CREATE TABLE IF NOT EXISTS system_meta (
        key TEXT PRIMARY KEY,
        value TEXT,
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
      );
    "#,
  },
  MigrationDef {
    description: "create_accounts_table",
    sql: r#"
      CREATE TABLE IF NOT EXISTS accounts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        code TEXT NOT NULL UNIQUE,
        name TEXT NOT NULL,
        category TEXT NOT NULL,
        parent_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL,
        is_active INTEGER NOT NULL DEFAULT 1,
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
      );

      CREATE INDEX IF NOT EXISTS idx_accounts_parent ON accounts(parent_id);
      CREATE INDEX IF NOT EXISTS idx_accounts_category ON accounts(category);
    "#,
  },
];

struct DatabaseState {
  path: PathBuf,
}

#[derive(Serialize)]
struct DatabaseInfo {
  url: &'static str,
  description: &'static str,
  path: String,
}

#[tauri::command]
fn get_database_info(state: tauri::State<DatabaseState>) -> DatabaseInfo {
  DatabaseInfo {
    url: DB_URL,
    description: DB_FILE_DESCRIPTION,
    path: state.path.to_string_lossy().to_string(),
  }
}

fn prepare_database_path(handle: &AppHandle) -> AnyResult<PathBuf> {
  let (_, relative_path) = DB_URL
    .split_once(':')
    .context("invalid database url, missing driver prefix")?;
  let mut app_config_dir = handle
    .path()
    .app_config_dir()
    .context("failed to resolve app config directory")?;

  app_config_dir.push(relative_path);

  if let Some(parent) = app_config_dir.parent() {
    fs::create_dir_all(parent)
      .with_context(|| format!("failed to create database parent directory {:?}", parent))?;
  }

  Ok(app_config_dir)
}

fn build_sql_plugin() -> tauri::plugin::TauriPlugin<tauri::Wry, Option<tauri_plugin_sql::PluginConfig>> {
  tauri_plugin_sql::Builder::new()
    .add_migrations(
      DB_URL,
      MIGRATION_DEFS
        .iter()
        .enumerate()
        .map(|(index, def)| Migration {
          version: (index + 1) as i64,
          description: def.description,
          sql: def.sql,
          kind: MigrationKind::Up,
        })
        .collect::<Vec<_>>(),
    )
    .build()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let db_path = prepare_database_path(&app.handle())?;

      #[cfg(feature = "tauri-plugin-log")]
      if cfg!(debug_assertions) {
        app
          .handle()
          .plugin(LogPluginBuilder::default().level(log::LevelFilter::Info).build())?;
      }

      app.handle().plugin(build_sql_plugin())?;
      app.manage(DatabaseState { path: db_path });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_database_info])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
