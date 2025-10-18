use std::{fs, path::Path, path::PathBuf};

use anyhow::{Context, Result as AnyResult};
use chrono::{NaiveDate, Utc};
use rusqlite::{params, params_from_iter, types::Value, Connection};
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

fn run_migrations(db_path: &Path) -> AnyResult<()> {
  let conn = Connection::open(db_path)
    .with_context(|| format!("unable to open database at {:?}", db_path))?;
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
      create_journal_entry
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

fn to_string<E: std::fmt::Display>(err: E) -> String {
  err.to_string()
}
