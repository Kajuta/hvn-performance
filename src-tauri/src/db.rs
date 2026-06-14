use chrono::Local;
use rusqlite::{Connection, Result, Transaction};
use uuid::Uuid;

pub fn get_connection() -> Result<Connection> {
    Connection::open("../hvn-performance.db")
}

pub fn init_db() -> Result<()> {
    let mut conn = get_connection()?;
    let tx = conn.transaction()?;

    create_fee_category_master_table(&tx)?;
    create_import_batches_table(&tx)?;
    create_visit_fee_records_table(&tx)?;
    create_fee_item_master_table(&tx)?;
    create_fee_master_table(&tx)?;

    tx.commit()?;
    Ok(())
}

fn create_fee_category_master_table(tx: &rusqlite::Transaction) -> Result<()> {
    tx.execute(
        "
        CREATE TABLE IF NOT EXISTS fee_category_master (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            category_name TEXT UNIQUE NOT NULL,
            group_name TEXT,
            item_type TEXT,
            display_order INTEGER,
            is_active INTEGER NOT NULL DEFAULT 1
        )
        ",
        [],
    )?;

    Ok(())
}

fn create_import_batches_table(tx: &rusqlite::Transaction) -> Result<()> {
    tx.execute(
        "
        CREATE TABLE IF NOT EXISTS import_batches (
            id TEXT PRIMARY KEY,
            target_month TEXT,
            source_file_name TEXT NOT NULL,
            imported_at TEXT NOT NULL,
            record_count INTEGER NOT NULL
        )
        ",
        [],
    )?;

    Ok(())
}

fn create_visit_fee_records_table(tx: &rusqlite::Transaction) -> Result<()> {
    tx.execute(
        "
        CREATE TABLE IF NOT EXISTS visit_fee_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            import_batch_id TEXT NOT NULL,
            user_name TEXT NOT NULL,
            user_id TEXT,
            fee_item_name TEXT NOT NULL,
            count INTEGER NOT NULL,
            source_row INTEGER,
            source_column INTEGER,
            FOREIGN KEY(import_batch_id)
                REFERENCES import_batches(id)
        )
        ",
        [],
    )?;

    Ok(())
}

fn create_fee_item_master_table(tx: &rusqlite::Transaction) -> Result<()> {
    tx.execute(
        "
        CREATE TABLE IF NOT EXISTS fee_item_master (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ibow_item_name TEXT UNIQUE NOT NULL,
            category_name TEXT NOT NULL,
            group_name TEXT,
            item_type TEXT,
            is_active INTEGER NOT NULL DEFAULT 1,
            display_order INTEGER
        )
        ",
        [],
    )?;

    Ok(())
}

fn create_fee_master_table(tx: &rusqlite::Transaction) -> Result<()> {
    tx.execute(
        "
        CREATE TABLE IF NOT EXISTS fee_master (
            fee_code TEXT PRIMARY KEY,
            fee_name TEXT NOT NULL,
            kana_name TEXT,
            unit_price INTEGER NOT NULL,
            valid_from TEXT,
            valid_to TEXT,
            raw_json TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
        )
        ",
        [],
    )?;

    Ok(())
}

pub fn create_import_batch(
    tx: &Transaction,
    target_month: Option<&str>,
    source_file_name: &str,
    record_count: usize,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    let imported_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    tx.execute(
        "
        INSERT INTO import_batches (
            id,
            target_month,
            source_file_name,
            imported_at,
            record_count
        )
        VALUES (?1, ?2, ?3, ?4, ?5)
        ",
        (
            &id,
            target_month,
            source_file_name,
            imported_at,
            record_count as i64,
        ),
    )?;

    Ok(id)
}