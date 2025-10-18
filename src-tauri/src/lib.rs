use std::{fs, path::Path, path::PathBuf};

use anyhow::{Context, Result as AnyResult};
use chrono::{Datelike, NaiveDate, Utc};
use rusqlite::{params, params_from_iter, types::Value, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
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
    description: "create_app_settings_table",
    sql: r#"
      CREATE TABLE IF NOT EXISTS app_settings (
        id INTEGER PRIMARY KEY CHECK (id = 1),
        company_name TEXT NOT NULL DEFAULT 'Perusahaan Tanpa Nama',
        company_email TEXT,
        company_phone TEXT,
        fiscal_year_start TEXT,
        default_currency TEXT NOT NULL DEFAULT 'IDR',
        timezone TEXT DEFAULT 'Asia/Jakarta',
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
      );

      INSERT OR IGNORE INTO app_settings (id) VALUES (1);
    "#,
  },
  MigrationDef {
    description: "create_accounts_table",
    sql: r#"
      CREATE TABLE IF NOT EXISTS accounts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        code TEXT NOT NULL UNIQUE,
        name TEXT NOT NULL,
        account_type TEXT NOT NULL CHECK (account_type IN ('ASSET','LIABILITY','EQUITY','REVENUE','EXPENSE')),
        sub_type TEXT,
        normal_balance TEXT NOT NULL CHECK (normal_balance IN ('DEBIT','CREDIT')),
        parent_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL,
        description TEXT,
        is_active INTEGER NOT NULL DEFAULT 1,
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
      );

      CREATE INDEX IF NOT EXISTS idx_accounts_parent ON accounts(parent_id);
      CREATE INDEX IF NOT EXISTS idx_accounts_type ON accounts(account_type);
    "#,
  },
  MigrationDef {
    description: "create_contacts_and_items_tables",
    sql: r#"
      CREATE TABLE IF NOT EXISTS contacts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        contact_type TEXT NOT NULL CHECK (contact_type IN ('CUSTOMER','SUPPLIER','OTHER')),
        email TEXT,
        phone TEXT,
        tax_id TEXT,
        address TEXT,
        city TEXT,
        state TEXT,
        postal_code TEXT,
        country TEXT,
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
      );

      CREATE INDEX IF NOT EXISTS idx_contacts_type ON contacts(contact_type);

      CREATE TABLE IF NOT EXISTS items (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        sku TEXT UNIQUE,
        item_type TEXT NOT NULL CHECK (item_type IN ('PRODUCT','SERVICE')),
        description TEXT,
        unit_price REAL NOT NULL DEFAULT 0,
        income_account_id INTEGER REFERENCES accounts(id),
        expense_account_id INTEGER REFERENCES accounts(id),
        inventory_account_id INTEGER REFERENCES accounts(id),
        is_active INTEGER NOT NULL DEFAULT 1,
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
      );

      CREATE INDEX IF NOT EXISTS idx_items_type ON items(item_type);
    "#,
  },
  MigrationDef {
    description: "create_invoice_tables",
    sql: r#"
      CREATE TABLE IF NOT EXISTS invoices (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        contact_id INTEGER NOT NULL REFERENCES contacts(id) ON DELETE RESTRICT,
        invoice_number TEXT NOT NULL UNIQUE,
        issue_date TEXT NOT NULL,
        due_date TEXT,
        status TEXT NOT NULL CHECK (status IN ('DRAFT','SENT','PARTIAL','PAID','VOID','OVERDUE')),
        currency TEXT NOT NULL DEFAULT 'IDR',
        total REAL NOT NULL DEFAULT 0,
        balance REAL NOT NULL DEFAULT 0,
        notes TEXT,
        reference TEXT,
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
      );

      CREATE INDEX IF NOT EXISTS idx_invoices_contact ON invoices(contact_id);
      CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);
      CREATE INDEX IF NOT EXISTS idx_invoices_due_date ON invoices(due_date);

      CREATE TABLE IF NOT EXISTS invoice_items (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        invoice_id INTEGER NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
        item_id INTEGER REFERENCES items(id),
        account_id INTEGER REFERENCES accounts(id),
        description TEXT,
        quantity REAL NOT NULL DEFAULT 1,
        unit_price REAL NOT NULL DEFAULT 0,
        amount REAL NOT NULL DEFAULT 0,
        tax_rate REAL NOT NULL DEFAULT 0
      );

      CREATE INDEX IF NOT EXISTS idx_invoice_items_invoice ON invoice_items(invoice_id);

      CREATE TABLE IF NOT EXISTS invoice_payments (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        invoice_id INTEGER NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
        payment_date TEXT NOT NULL,
        amount REAL NOT NULL,
        method TEXT,
        reference TEXT,
        notes TEXT,
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
      );

      CREATE INDEX IF NOT EXISTS idx_invoice_payments_invoice ON invoice_payments(invoice_id);
    "#,
  },
  MigrationDef {
    description: "create_journal_tables",
    sql: r#"
      CREATE TABLE IF NOT EXISTS journals (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        journal_number TEXT UNIQUE,
        journal_date TEXT NOT NULL,
        memo TEXT,
        source TEXT,
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
      );

      CREATE INDEX IF NOT EXISTS idx_journals_date ON journals(journal_date);

      CREATE TABLE IF NOT EXISTS journal_lines (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        journal_id INTEGER NOT NULL REFERENCES journals(id) ON DELETE CASCADE,
        account_id INTEGER NOT NULL REFERENCES accounts(id),
        description TEXT,
        debit REAL NOT NULL DEFAULT 0,
        credit REAL NOT NULL DEFAULT 0,
        contact_id INTEGER REFERENCES contacts(id),
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
      );

      CREATE INDEX IF NOT EXISTS idx_journal_lines_journal ON journal_lines(journal_id);
      CREATE INDEX IF NOT EXISTS idx_journal_lines_account ON journal_lines(account_id);
    "#,
  },
  MigrationDef {
    description: "seed_default_chart_of_accounts",
    sql: r#"
      INSERT OR IGNORE INTO accounts (code, name, account_type, sub_type, normal_balance, parent_id)
      VALUES
        ('1000', 'Aktiva Lancar', 'ASSET', 'CURRENT_ASSET', 'DEBIT', NULL),
        ('1100', 'Kas', 'ASSET', 'CASH', 'DEBIT', (SELECT id FROM accounts WHERE code = '1000')),
        ('1101', 'Kas Besar', 'ASSET', 'CASH', 'DEBIT', (SELECT id FROM accounts WHERE code = '1100')),
        ('1102', 'Bank', 'ASSET', 'CASH', 'DEBIT', (SELECT id FROM accounts WHERE code = '1100')),
        ('1200', 'Piutang Usaha', 'ASSET', 'RECEIVABLE', 'DEBIT', (SELECT id FROM accounts WHERE code = '1000')),
        ('1300', 'Persediaan', 'ASSET', 'INVENTORY', 'DEBIT', (SELECT id FROM accounts WHERE code = '1000')),
        ('1400', 'Uang Muka', 'ASSET', 'PREPAID', 'DEBIT', (SELECT id FROM accounts WHERE code = '1000')),
        ('1500', 'Aset Tetap', 'ASSET', 'FIXED_ASSET', 'DEBIT', NULL),
        ('1510', 'Peralatan', 'ASSET', 'FIXED_ASSET', 'DEBIT', (SELECT id FROM accounts WHERE code = '1500')),
        ('1520', 'Akumulasi Penyusutan', 'ASSET', 'ACCUMULATED_DEPRECIATION', 'CREDIT', (SELECT id FROM accounts WHERE code = '1500')),

        ('2000', 'Kewajiban Lancar', 'LIABILITY', 'CURRENT_LIABILITY', 'CREDIT', NULL),
        ('2100', 'Utang Usaha', 'LIABILITY', 'PAYABLE', 'CREDIT', (SELECT id FROM accounts WHERE code = '2000')),
        ('2200', 'Utang Pajak', 'LIABILITY', 'TAX_PAYABLE', 'CREDIT', (SELECT id FROM accounts WHERE code = '2000')),
        ('2300', 'Pendapatan Diterima Dimuka', 'LIABILITY', 'UNEARNED_REVENUE', 'CREDIT', (SELECT id FROM accounts WHERE code = '2000')),

        ('3000', 'Ekuitas', 'EQUITY', 'EQUITY', 'CREDIT', NULL),
        ('3100', 'Modal Pemilik', 'EQUITY', 'OWNER_EQUITY', 'CREDIT', (SELECT id FROM accounts WHERE code = '3000')),
        ('3200', 'Laba Ditahan', 'EQUITY', 'RETAINED_EARNINGS', 'CREDIT', (SELECT id FROM accounts WHERE code = '3000')),

        ('4000', 'Pendapatan Operasional', 'REVENUE', 'OPERATING_REVENUE', 'CREDIT', NULL),
        ('4100', 'Pendapatan Jasa', 'REVENUE', 'SERVICE_REVENUE', 'CREDIT', (SELECT id FROM accounts WHERE code = '4000')),
        ('4200', 'Pendapatan Lain-lain', 'REVENUE', 'OTHER_REVENUE', 'CREDIT', (SELECT id FROM accounts WHERE code = '4000')),

        ('5000', 'Harga Pokok Penjualan', 'EXPENSE', 'COGS', 'DEBIT', NULL),
        ('5100', 'Beban Pokok Penjualan', 'EXPENSE', 'COGS', 'DEBIT', (SELECT id FROM accounts WHERE code = '5000')),

        ('6000', 'Beban Operasional', 'EXPENSE', 'OPERATING_EXPENSE', 'DEBIT', NULL),
        ('6100', 'Beban Gaji', 'EXPENSE', 'OPERATING_EXPENSE', 'DEBIT', (SELECT id FROM accounts WHERE code = '6000')),
        ('6200', 'Beban Sewa', 'EXPENSE', 'OPERATING_EXPENSE', 'DEBIT', (SELECT id FROM accounts WHERE code = '6000')),
        ('6300', 'Beban Utilitas', 'EXPENSE', 'OPERATING_EXPENSE', 'DEBIT', (SELECT id FROM accounts WHERE code = '6000')),
        ('6400', 'Beban Administrasi & Umum', 'EXPENSE', 'OPERATING_EXPENSE', 'DEBIT', (SELECT id FROM accounts WHERE code = '6000')),
        ('6500', 'Beban Pemasaran', 'EXPENSE', 'OPERATING_EXPENSE', 'DEBIT', (SELECT id FROM accounts WHERE code = '6000'));

      INSERT OR IGNORE INTO system_meta (key, value)
      VALUES ('db_version', '1');
    "#,
  },
];

struct DatabaseState {
  path: PathBuf,
}

impl DatabaseState {
  fn open_connection(&self) -> AnyResult<Connection> {
    let conn = Connection::open(&self.path)
      .with_context(|| format!("failed to open sqlite connection at {:?}", self.path))?;
    conn
      .execute_batch(
        "
          PRAGMA foreign_keys = ON;
          PRAGMA busy_timeout = 5000;
        ",
      )
      .context("failed to configure sqlite pragmas")?;
    Ok(conn)
  }
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

fn maybe_upgrade_legacy_accounts_schema(conn: &Connection) -> AnyResult<()> {
  let mut stmt = conn
    .prepare("PRAGMA table_info(accounts)")
    .context("failed to inspect accounts table info")?;
  let columns = stmt
    .query_map([], |row| row.get::<_, String>(1))?
    .collect::<rusqlite::Result<Vec<_>>>()
    .context("failed to read accounts table columns")?;

  let has_category = columns.iter().any(|name| name == "category");
  let has_account_type = columns.iter().any(|name| name == "account_type");

  if has_category && !has_account_type {
    conn
      .execute_batch("PRAGMA foreign_keys = OFF;")
      .context("failed to disable foreign keys before legacy accounts upgrade")?;

    conn
      .execute_batch(
        "
          BEGIN TRANSACTION;

          ALTER TABLE accounts RENAME TO accounts_old;

          CREATE TABLE accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            code TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            account_type TEXT NOT NULL CHECK (account_type IN ('ASSET','LIABILITY','EQUITY','REVENUE','EXPENSE')),
            sub_type TEXT,
            normal_balance TEXT NOT NULL CHECK (normal_balance IN ('DEBIT','CREDIT')),
            parent_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL,
            description TEXT,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
          );

          INSERT INTO accounts (
            id,
            code,
            name,
            account_type,
            sub_type,
            normal_balance,
            parent_id,
            description,
            is_active,
            created_at,
            updated_at
          )
          SELECT
            id,
            code,
            name,
            CASE
              WHEN category IN ('ASSET','LIABILITY','EQUITY','REVENUE','EXPENSE') THEN category
              ELSE 'ASSET'
            END AS account_type,
            NULL AS sub_type,
            CASE
              WHEN category IN ('LIABILITY','EQUITY','REVENUE') THEN 'CREDIT'
              ELSE 'DEBIT'
            END AS normal_balance,
            parent_id,
            NULL AS description,
            is_active,
            created_at,
            updated_at
          FROM accounts_old;

          DROP TABLE accounts_old;

          CREATE INDEX IF NOT EXISTS idx_accounts_parent ON accounts(parent_id);
          CREATE INDEX IF NOT EXISTS idx_accounts_type ON accounts(account_type);

          COMMIT;
        ",
      )
      .context("failed to upgrade legacy accounts table schema")?;

    conn
      .execute_batch("PRAGMA foreign_keys = ON;")
      .context("failed to re-enable foreign keys after legacy accounts upgrade")?;
  }

  Ok(())
}

fn run_migrations(db_path: &Path) -> AnyResult<()> {
  let conn = Connection::open(db_path)
    .with_context(|| format!("unable to open database at {:?}", db_path))?;
  maybe_upgrade_legacy_accounts_schema(&conn)?;
  conn
    .execute_batch("PRAGMA foreign_keys = ON;")
    .context("failed to enable foreign keys during migration")?;

  for migration in MIGRATION_DEFS {
    conn
      .execute_batch(migration.sql)
      .with_context(|| format!("failed to run migration: {}", migration.description))?;
  }

  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let db_path = prepare_database_path(&app.handle())?;
      run_migrations(&db_path)?;

      #[cfg(feature = "tauri-plugin-log")]
      if cfg!(debug_assertions) {
        app
          .handle()
          .plugin(LogPluginBuilder::default().level(log::LevelFilter::Info).build())?;
      }

      app.manage(DatabaseState { path: db_path });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      get_database_info,
      list_accounts,
      create_account,
      list_journals,
      create_journal_entry,
      generate_balance_sheet,
      generate_income_statement
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Serialize)]
struct AccountDto {
  id: i64,
  code: String,
  name: String,
  account_type: String,
  sub_type: Option<String>,
  normal_balance: String,
  parent_id: Option<i64>,
  description: Option<String>,
  is_active: bool,
}

#[derive(Deserialize)]
struct CreateAccountPayload {
  code: String,
  name: String,
  account_type: String,
  sub_type: Option<String>,
  normal_balance: String,
  parent_id: Option<i64>,
  description: Option<String>,
  is_active: Option<bool>,
}

#[tauri::command]
fn list_accounts(state: tauri::State<DatabaseState>) -> Result<Vec<AccountDto>, String> {
  let conn = state.open_connection().map_err(to_string)?;
  let mut stmt = conn
    .prepare(
      "
        SELECT
          id,
          code,
          name,
          account_type,
          sub_type,
          normal_balance,
          parent_id,
          description,
          is_active
        FROM accounts
        ORDER BY code ASC
      ",
    )
    .map_err(to_string)?;

  let rows = stmt
    .query_map([], |row| {
      Ok(AccountDto {
        id: row.get(0)?,
        code: row.get(1)?,
        name: row.get(2)?,
        account_type: row.get(3)?,
        sub_type: row.get(4)?,
        normal_balance: row.get(5)?,
        parent_id: row.get(6)?,
        description: row.get(7)?,
        is_active: row.get::<_, i64>(8)? == 1,
      })
    })
    .map_err(to_string)?;

  let mut accounts = Vec::new();
  for account in rows {
    accounts.push(account.map_err(to_string)?);
  }

  Ok(accounts)
}

#[tauri::command]
fn create_account(
  state: tauri::State<DatabaseState>,
  payload: CreateAccountPayload,
) -> Result<AccountDto, String> {
  let mut conn = state.open_connection().map_err(to_string)?;
  let tx = conn
    .transaction()
    .map_err(|err| format!("gagal memulai transaksi: {err}"))?;

  tx.execute(
    "
      INSERT INTO accounts (
        code,
        name,
        account_type,
        sub_type,
        normal_balance,
        parent_id,
        description,
        is_active
      )
      VALUES (?1, ?2, UPPER(?3), ?4, UPPER(?5), ?6, ?7, ?8)
    ",
    params![
      payload.code.trim(),
      payload.name.trim(),
      payload.account_type.trim(),
      payload.sub_type.as_deref(),
      payload.normal_balance.trim(),
      payload.parent_id,
      payload.description.as_deref(),
      payload.is_active.unwrap_or(true) as i64
    ],
  )
  .map_err(|err| format!("gagal menambahkan akun baru: {err}"))?;

  let account_id = tx.last_insert_rowid();
  let account = tx
    .query_row(
      "
        SELECT id, code, name, account_type, sub_type, normal_balance, parent_id, description, is_active
        FROM accounts
        WHERE id = ?1
      ",
      params![account_id],
      |row| {
        Ok(AccountDto {
          id: row.get(0)?,
          code: row.get(1)?,
          name: row.get(2)?,
          account_type: row.get(3)?,
          sub_type: row.get(4)?,
          normal_balance: row.get(5)?,
          parent_id: row.get(6)?,
          description: row.get(7)?,
          is_active: row.get::<_, i64>(8)? == 1,
        })
      },
    )
    .map_err(|err| format!("gagal mengambil akun baru: {err}"))?;

  tx.commit()
    .map_err(|err| format!("gagal menyimpan akun baru: {err}"))?;

  Ok(account)
}

#[derive(Default, Deserialize)]
struct ListJournalParams {
  limit: Option<u32>,
  offset: Option<u32>,
  start_date: Option<String>,
  end_date: Option<String>,
  search: Option<String>,
}

#[derive(Serialize)]
struct JournalSummary {
  id: i64,
  journal_number: Option<String>,
  journal_date: String,
  memo: Option<String>,
  source: Option<String>,
  total_debit: f64,
  total_credit: f64,
}

#[tauri::command]
fn list_journals(
  state: tauri::State<DatabaseState>,
  params: Option<ListJournalParams>,
) -> Result<Vec<JournalSummary>, String> {
  let conn = state.open_connection().map_err(to_string)?;
  let params = params.unwrap_or_default();

  let limit = params.limit.unwrap_or(50).min(200) as i64;
  let offset = params.offset.unwrap_or(0) as i64;

  let mut conditions = Vec::new();
  let mut bindings: Vec<Value> = Vec::new();

  if let Some(start) = params.start_date {
    conditions.push("journal_date >= ?".to_string());
    bindings.push(Value::from(start));
  }

  if let Some(end) = params.end_date {
    conditions.push("journal_date <= ?".to_string());
    bindings.push(Value::from(end));
  }

  if let Some(search) = params.search {
    let like = format!("%{}%", search);
    conditions.push("(journal_number LIKE ? OR memo LIKE ?)".to_string());
    bindings.push(Value::from(like.clone()));
    bindings.push(Value::from(like));
  }

  let mut sql = String::from(
    "
      SELECT
        j.id,
        j.journal_number,
        j.journal_date,
        j.memo,
        j.source,
        IFNULL(SUM(l.debit), 0) AS total_debit,
        IFNULL(SUM(l.credit), 0) AS total_credit
      FROM journals j
      LEFT JOIN journal_lines l ON l.journal_id = j.id
    ",
  );

  if !conditions.is_empty() {
    sql.push_str(" WHERE ");
    sql.push_str(&conditions.join(" AND "));
  }

  sql.push_str(
    "
      GROUP BY j.id
      ORDER BY j.journal_date DESC, j.id DESC
      LIMIT ? OFFSET ?
    ",
  );

  bindings.push(Value::from(limit));
  bindings.push(Value::from(offset));

  let mut stmt = conn.prepare(&sql).map_err(to_string)?;
  let mut rows = stmt
    .query(params_from_iter(bindings.iter()))
    .map_err(to_string)?;

  let mut journals = Vec::new();
  while let Some(row) = rows.next().map_err(to_string)? {
    journals.push(JournalSummary {
      id: row.get(0).map_err(to_string)?,
      journal_number: row.get(1).map_err(to_string)?,
      journal_date: row.get(2).map_err(to_string)?,
      memo: row.get(3).map_err(to_string)?,
      source: row.get(4).map_err(to_string)?,
      total_debit: row.get(5).map_err(to_string)?,
      total_credit: row.get(6).map_err(to_string)?,
    });
  }

  Ok(journals)
}

#[derive(Deserialize)]
struct JournalLinePayload {
  account_id: i64,
  description: Option<String>,
  debit: f64,
  credit: f64,
  contact_id: Option<i64>,
}

#[derive(Deserialize)]
struct CreateJournalPayload {
  journal_number: Option<String>,
  journal_date: String,
  memo: Option<String>,
  source: Option<String>,
  lines: Vec<JournalLinePayload>,
}

#[tauri::command]
fn create_journal_entry(
  state: tauri::State<DatabaseState>,
  payload: CreateJournalPayload,
) -> Result<JournalSummary, String> {
  let CreateJournalPayload {
    journal_number,
    journal_date,
    memo,
    source,
    lines,
  } = payload;

  if lines.len() < 2 {
    return Err("Minimal dua baris jurnal diperlukan.".into());
  }

  let total_debit: f64 = lines.iter().map(|line| line.debit).sum();
  let total_credit: f64 = lines.iter().map(|line| line.credit).sum();
  if (total_debit - total_credit).abs() > 0.01 {
    return Err(format!(
      "Total debit ({total_debit:.2}) dan kredit ({total_credit:.2}) harus seimbang."
    ));
  }

  NaiveDate::parse_from_str(&journal_date, "%Y-%m-%d")
    .map_err(|_| "Format tanggal jurnal tidak valid. Gunakan YYYY-MM-DD.".to_string())?;

  let mut conn = state.open_connection().map_err(to_string)?;
  let tx = conn
    .transaction()
    .map_err(|err| format!("gagal memulai transaksi jurnal: {err}"))?;

  let journal_number = match journal_number {
    Some(number) => number,
    None => {
      let seq: i64 = tx
        .query_row("SELECT COALESCE(MAX(id), 0) + 1 FROM journals", [], |row| row.get(0))
        .map_err(|err| format!("gagal menghitung nomor jurnal: {err}"))?;
      format!("JRN-{:05}", seq)
    }
  };

  let created_at = Utc::now().to_rfc3339();

  tx.execute(
    "
      INSERT INTO journals (journal_number, journal_date, memo, source, created_at)
      VALUES (?1, ?2, ?3, ?4, ?5)
    ",
    params![
      journal_number.trim(),
      &journal_date,
      memo.as_deref(),
      source.as_deref(),
      created_at.as_str()
    ],
  )
  .map_err(|err| format!("gagal menyimpan jurnal: {err}"))?;

  let journal_id = tx.last_insert_rowid();

  {
    let mut line_stmt = tx
      .prepare(
        "
          INSERT INTO journal_lines (
            journal_id,
            account_id,
            description,
            debit,
            credit,
            contact_id,
            created_at
          )
          VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ",
      )
      .map_err(|err| format!("gagal menyiapkan pernyataan jurnal detail: {err}"))?;

    for line in &lines {
      if line.debit < 0.0 || line.credit < 0.0 {
        return Err("Nilai debit/kredit tidak boleh negatif.".into());
      }

      line_stmt
        .execute(params![
          journal_id,
          line.account_id,
          line.description.as_deref(),
          line.debit,
          line.credit,
          line.contact_id,
          Utc::now().to_rfc3339(),
        ])
        .map_err(|err| format!("gagal menyimpan baris jurnal: {err}"))?;
    }
  }

  tx.commit()
    .map_err(|err| format!("gagal menyimpan jurnal: {err}"))?;

  let summary = conn
    .query_row(
      "
        SELECT
          j.id,
          j.journal_number,
          j.journal_date,
          j.memo,
          j.source,
          IFNULL(SUM(l.debit), 0),
          IFNULL(SUM(l.credit), 0)
        FROM journals j
        LEFT JOIN journal_lines l ON l.journal_id = j.id
        WHERE j.id = ?1
        GROUP BY j.id
      ",
      params![journal_id],
      |row| {
        Ok(JournalSummary {
          id: row.get(0)?,
          journal_number: row.get(1)?,
          journal_date: row.get(2)?,
          memo: row.get(3)?,
          source: row.get(4)?,
          total_debit: row.get(5)?,
          total_credit: row.get(6)?,
        })
      },
    )
    .map_err(|err| format!("gagal mengambil ringkasan jurnal: {err}"))?;

  Ok(summary)
}

#[derive(Default, Deserialize)]
struct BalanceSheetParams {
  as_of: Option<String>,
}

#[derive(Default, Deserialize)]
struct IncomeStatementParams {
  start_date: Option<String>,
  end_date: Option<String>,
}

#[derive(Serialize)]
struct AccountBalanceRow {
  account_id: i64,
  parent_id: Option<i64>,
  code: String,
  name: String,
  account_type: String,
  normal_balance: String,
  balance: f64,
}

#[derive(Serialize)]
struct BalanceSheetSection {
  key: String,
  label: String,
  total: f64,
  accounts: Vec<AccountBalanceRow>,
}

#[derive(Serialize)]
struct BalanceSheetResponse {
  as_of: String,
  currency: String,
  sections: Vec<BalanceSheetSection>,
  total_liabilities_and_equity: f64,
  generated_at: String,
}

#[derive(Serialize)]
struct IncomeStatementSection {
  key: String,
  label: String,
  total: f64,
  accounts: Vec<AccountBalanceRow>,
}

#[derive(Serialize)]
struct IncomeStatementTotals {
  total_revenue: f64,
  total_expenses: f64,
  net_income: f64,
}

#[derive(Serialize)]
struct IncomeStatementResponse {
  start_date: String,
  end_date: String,
  currency: String,
  sections: Vec<IncomeStatementSection>,
  totals: IncomeStatementTotals,
  generated_at: String,
}

#[tauri::command]
fn generate_balance_sheet(
  state: tauri::State<DatabaseState>,
  params: Option<BalanceSheetParams>,
) -> Result<BalanceSheetResponse, String> {
  let params = params.unwrap_or_default();
  let as_of = match params.as_of {
    Some(date_str) => NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
      .map_err(|_| "Format tanggal tidak valid. Gunakan YYYY-MM-DD.".to_string())?,
    None => Utc::now().date_naive(),
  };

  let conn = state.open_connection().map_err(to_string)?;
  compute_balance_sheet(&conn, as_of).map_err(to_string)
}

#[tauri::command]
fn generate_income_statement(
  state: tauri::State<DatabaseState>,
  params: Option<IncomeStatementParams>,
) -> Result<IncomeStatementResponse, String> {
  let params = params.unwrap_or_default();
  let end_date = match params.end_date {
    Some(date_str) => NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
      .map_err(|_| "Format tanggal akhir tidak valid. Gunakan YYYY-MM-DD.".to_string())?,
    None => Utc::now().date_naive(),
  };
  let start_date = match params.start_date {
    Some(date_str) => NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
      .map_err(|_| "Format tanggal mulai tidak valid. Gunakan YYYY-MM-DD.".to_string())?,
    None => NaiveDate::from_ymd_opt(end_date.year(), 1, 1).unwrap_or(end_date),
  };

  if start_date > end_date {
    return Err("Tanggal mulai tidak boleh setelah tanggal akhir.".into());
  }

  let conn = state.open_connection().map_err(to_string)?;
  compute_income_statement(&conn, start_date, end_date).map_err(to_string)
}

fn compute_balance_sheet(conn: &Connection, as_of: NaiveDate) -> AnyResult<BalanceSheetResponse> {
  let currency = fetch_default_currency(conn)?;
  let accounts = load_account_balances(
    conn,
    Some(&["ASSET", "LIABILITY", "EQUITY"]),
    earliest_supported_date(),
    as_of,
  )?;

  let mut assets = Vec::new();
  let mut liabilities = Vec::new();
  let mut equity = Vec::new();
  let mut total_assets = 0.0;
  let mut total_liabilities = 0.0;
  let mut total_equity = 0.0;

  for account in accounts {
    match account.account_type.as_str() {
      "ASSET" => {
        total_assets += account.balance;
        assets.push(account);
      }
      "LIABILITY" => {
        total_liabilities += account.balance;
        liabilities.push(account);
      }
      "EQUITY" => {
        total_equity += account.balance;
        equity.push(account);
      }
      _ => {}
    }
  }

  let total_liabilities_and_equity = normalize_amount(total_liabilities + total_equity);

  Ok(BalanceSheetResponse {
    as_of: as_of.format("%Y-%m-%d").to_string(),
    currency,
    sections: vec![
      BalanceSheetSection {
        key: "ASSET".to_string(),
        label: "Aset".to_string(),
        total: normalize_amount(total_assets),
        accounts: assets,
      },
      BalanceSheetSection {
        key: "LIABILITY".to_string(),
        label: "Kewajiban".to_string(),
        total: normalize_amount(total_liabilities),
        accounts: liabilities,
      },
      BalanceSheetSection {
        key: "EQUITY".to_string(),
        label: "Ekuitas".to_string(),
        total: normalize_amount(total_equity),
        accounts: equity,
      },
    ],
    total_liabilities_and_equity,
    generated_at: Utc::now().to_rfc3339(),
  })
}

fn compute_income_statement(
  conn: &Connection,
  start_date: NaiveDate,
  end_date: NaiveDate,
) -> AnyResult<IncomeStatementResponse> {
  let currency = fetch_default_currency(conn)?;
  let accounts = load_account_balances(
    conn,
    Some(&["REVENUE", "EXPENSE"]),
    start_date,
    end_date,
  )?;

  let mut revenues = Vec::new();
  let mut expenses = Vec::new();
  let mut revenue_total = 0.0;
  let mut expense_total = 0.0;

  for account in accounts {
    match account.account_type.as_str() {
      "REVENUE" => {
        revenue_total += account.balance;
        revenues.push(account);
      }
      "EXPENSE" => {
        expense_total += account.balance;
        expenses.push(account);
      }
      _ => {}
    }
  }

  let total_revenue = normalize_amount(revenue_total);
  let total_expenses = normalize_amount(expense_total);
  let net_income = normalize_amount(total_revenue - total_expenses);

  Ok(IncomeStatementResponse {
    start_date: start_date.format("%Y-%m-%d").to_string(),
    end_date: end_date.format("%Y-%m-%d").to_string(),
    currency,
    sections: vec![
      IncomeStatementSection {
        key: "REVENUE".to_string(),
        label: "Pendapatan".to_string(),
        total: total_revenue,
        accounts: revenues,
      },
      IncomeStatementSection {
        key: "EXPENSE".to_string(),
        label: "Beban".to_string(),
        total: total_expenses,
        accounts: expenses,
      },
    ],
    totals: IncomeStatementTotals {
      total_revenue,
      total_expenses,
      net_income,
    },
    generated_at: Utc::now().to_rfc3339(),
  })
}

fn load_account_balances(
  conn: &Connection,
  account_types: Option<&[&str]>,
  start_date: NaiveDate,
  end_date: NaiveDate,
) -> AnyResult<Vec<AccountBalanceRow>> {
  let start_str = start_date.format("%Y-%m-%d").to_string();
  let end_str = end_date.format("%Y-%m-%d").to_string();

  let mut sql = String::from(
    "
      SELECT
        a.id,
        a.parent_id,
        a.code,
        a.name,
        a.account_type,
        a.normal_balance,
        IFNULL(SUM(CASE WHEN j.journal_date BETWEEN ? AND ? THEN jl.debit ELSE 0 END), 0) AS total_debit,
        IFNULL(SUM(CASE WHEN j.journal_date BETWEEN ? AND ? THEN jl.credit ELSE 0 END), 0) AS total_credit
      FROM accounts a
      LEFT JOIN journal_lines jl ON jl.account_id = a.id
      LEFT JOIN journals j ON j.id = jl.journal_id
    ",
  );

  let mut bindings: Vec<Value> = Vec::new();
  bindings.push(Value::from(start_str.clone()));
  bindings.push(Value::from(end_str.clone()));
  bindings.push(Value::from(start_str));
  bindings.push(Value::from(end_str.clone()));

  if let Some(types) = account_types {
    if !types.is_empty() {
      sql.push_str(" WHERE a.account_type IN (");
      for (index, _) in types.iter().enumerate() {
        if index > 0 {
          sql.push_str(", ");
        }
        sql.push('?');
      }
      sql.push(')');
      for kind in types {
        bindings.push(Value::from((*kind).to_string()));
      }
    }
  }

  sql.push_str(" GROUP BY a.id ORDER BY a.code ASC");

  let mut stmt = conn.prepare(&sql)?;
  let mut rows = stmt.query(params_from_iter(bindings.iter()))?;

  let mut accounts = Vec::new();
  while let Some(row) = rows.next()? {
    let normal_balance: String = row.get(5)?;
    let total_debit: f64 = row.get(6)?;
    let total_credit: f64 = row.get(7)?;
    let balance = if normal_balance == "DEBIT" {
      total_debit - total_credit
    } else {
      total_credit - total_debit
    };
    let balance = normalize_amount(balance);

    accounts.push(AccountBalanceRow {
      account_id: row.get(0)?,
      parent_id: row.get(1)?,
      code: row.get(2)?,
      name: row.get(3)?,
      account_type: row.get(4)?,
      normal_balance,
      balance,
    });
  }

  Ok(accounts)
}

fn fetch_default_currency(conn: &Connection) -> AnyResult<String> {
  let currency: Option<String> = conn
    .query_row(
      "SELECT default_currency FROM app_settings WHERE id = 1",
      [],
      |row| row.get(0),
    )
    .optional()?;

  Ok(currency.filter(|value| !value.is_empty()).unwrap_or_else(|| "IDR".to_string()))
}

fn earliest_supported_date() -> NaiveDate {
  NaiveDate::from_ymd_opt(1900, 1, 1).expect("tanggal default valid")
}

fn normalize_amount(value: f64) -> f64 {
  let rounded = (value * 100.0).round() / 100.0;
  if rounded.abs() < 0.005 {
    0.0
  } else {
    rounded
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::NaiveDate;
  use rusqlite::Connection;

  fn setup_connection() -> Connection {
    let conn = Connection::open_in_memory().expect("gagal membuka database in-memory");
    conn
      .execute_batch("PRAGMA foreign_keys = ON;")
      .expect("gagal mengaktifkan foreign keys");
    for migration in super::MIGRATION_DEFS {
      conn
        .execute_batch(migration.sql)
        .expect("gagal menjalankan migrasi");
    }
    conn
  }

  fn account_id_by_code(conn: &Connection, code: &str) -> i64 {
    conn
      .query_row("SELECT id FROM accounts WHERE code = ?1", [code], |row| row.get(0))
      .expect("kode akun tidak ditemukan")
  }

  #[test]
  fn legacy_accounts_schema_is_upgraded() {
    let conn = Connection::open_in_memory().expect("gagal membuka database in-memory");
    conn
      .execute_batch(
        "
          CREATE TABLE accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            code TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            parent_id INTEGER,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
          );

          CREATE INDEX idx_accounts_category ON accounts(category);
          CREATE INDEX idx_accounts_parent ON accounts(parent_id);

          INSERT INTO accounts (code, name, category)
          VALUES
            ('1000', 'Aktiva Lancar', 'ASSET'),
            ('2000', 'Kewajiban Lancar', 'LIABILITY');
        ",
      )
      .expect("gagal membuat skema akun legacy");

    super::maybe_upgrade_legacy_accounts_schema(&conn)
      .expect("upgrade skema akun legacy gagal");

    let mut stmt = conn
      .prepare("PRAGMA table_info(accounts)")
      .expect("gagal membaca skema akun baru");
    let columns = stmt
      .query_map([], |row| row.get::<_, String>(1))
      .expect("gagal memetakan kolom")
      .collect::<rusqlite::Result<Vec<_>>>()
      .expect("gagal mengumpulkan daftar kolom");

    assert!(columns.contains(&"account_type".to_string()));
    assert!(columns.contains(&"normal_balance".to_string()));
    assert!(!columns.contains(&"category".to_string()));

    let (account_type, normal_balance): (String, String) = conn
      .query_row(
        "
          SELECT account_type, normal_balance
          FROM accounts
          WHERE code = '2000'
        ",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
      )
      .expect("gagal membaca data akun hasil upgrade");

    assert_eq!(account_type, "LIABILITY");
    assert_eq!(normal_balance, "CREDIT");

    let create_accounts_migration = super::MIGRATION_DEFS
      .iter()
      .find(|migration| migration.description == "create_accounts_table")
      .expect("migrasi accounts tersedia");

    conn
      .execute_batch(create_accounts_migration.sql)
      .expect("migrasi accounts seharusnya idempotent setelah upgrade");
  }

  #[test]
  fn balance_sheet_totals_follow_accounting_equation() {
    let conn = setup_connection();

    conn
      .execute(
        "
          INSERT INTO journals (journal_number, journal_date, memo, source)
          VALUES (?1, ?2, ?3, ?4)
        ",
        params!["JRN-TEST-001", "2025-01-15", "Setoran modal awal", "tests"],
      )
      .unwrap();
    let journal_id = conn.last_insert_rowid();

    let cash_id = account_id_by_code(&conn, "1100");
    let equity_id = account_id_by_code(&conn, "3100");

    conn
      .execute(
        "
          INSERT INTO journal_lines (journal_id, account_id, debit, credit)
          VALUES (?1, ?2, ?3, ?4)
        ",
        params![journal_id, cash_id, 5_000_000.0, 0.0],
      )
      .unwrap();

    conn
      .execute(
        "
          INSERT INTO journal_lines (journal_id, account_id, debit, credit)
          VALUES (?1, ?2, ?3, ?4)
        ",
        params![journal_id, equity_id, 0.0, 5_000_000.0],
      )
      .unwrap();

    let report =
      compute_balance_sheet(&conn, NaiveDate::from_ymd_opt(2025, 1, 31).unwrap()).unwrap();

    let assets = report
      .sections
      .iter()
      .find(|section| section.key == "ASSET")
      .expect("section aset tersedia");
    let equity = report
      .sections
      .iter()
      .find(|section| section.key == "EQUITY")
      .expect("section ekuitas tersedia");

    assert_eq!(assets.total, 5_000_000.0);
    assert_eq!(equity.total, 5_000_000.0);
    assert_eq!(report.total_liabilities_and_equity, 5_000_000.0);
  }

  #[test]
  fn income_statement_net_income_is_computed_correctly() {
    let conn = setup_connection();

    conn
      .execute(
        "
          INSERT INTO journals (journal_number, journal_date, memo, source)
          VALUES (?1, ?2, ?3, ?4)
        ",
        params!["JRN-TEST-002", "2025-02-05", "Pendapatan jasa", "tests"],
      )
      .unwrap();
    let revenue_journal_id = conn.last_insert_rowid();

    let cash_id = account_id_by_code(&conn, "1100");
    let revenue_id = account_id_by_code(&conn, "4100");

    conn
      .execute(
        "
          INSERT INTO journal_lines (journal_id, account_id, debit, credit)
          VALUES (?1, ?2, ?3, ?4)
        ",
        params![revenue_journal_id, cash_id, 2_000_000.0, 0.0],
      )
      .unwrap();

    conn
      .execute(
        "
          INSERT INTO journal_lines (journal_id, account_id, debit, credit)
          VALUES (?1, ?2, ?3, ?4)
        ",
        params![revenue_journal_id, revenue_id, 0.0, 2_000_000.0],
      )
      .unwrap();

    conn
      .execute(
        "
          INSERT INTO journals (journal_number, journal_date, memo, source)
          VALUES (?1, ?2, ?3, ?4)
        ",
        params!["JRN-TEST-003", "2025-02-10", "Pembayaran gaji", "tests"],
      )
      .unwrap();
    let expense_journal_id = conn.last_insert_rowid();

    let expense_id = account_id_by_code(&conn, "6100");

    conn
      .execute(
        "
          INSERT INTO journal_lines (journal_id, account_id, debit, credit)
          VALUES (?1, ?2, ?3, ?4)
        ",
        params![expense_journal_id, expense_id, 750_000.0, 0.0],
      )
      .unwrap();

    conn
      .execute(
        "
          INSERT INTO journal_lines (journal_id, account_id, debit, credit)
          VALUES (?1, ?2, ?3, ?4)
        ",
        params![expense_journal_id, cash_id, 0.0, 750_000.0],
      )
      .unwrap();

    let report = compute_income_statement(
      &conn,
      NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
      NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
    )
    .unwrap();

    assert_eq!(report.totals.total_revenue, 2_000_000.0);
    assert_eq!(report.totals.total_expenses, 750_000.0);
    assert_eq!(report.totals.net_income, 1_250_000.0);
  }
}

fn to_string<E: std::fmt::Display>(err: E) -> String {
  err.to_string()
}
