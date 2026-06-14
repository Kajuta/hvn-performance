use chrono::Local;
use rusqlite::{Connection, Result};
use uuid::Uuid;

pub fn get_connection() -> Result<Connection> {
    let conn = Connection::open("../hvn-performance.db")?;

    conn.execute(
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

    conn.execute(
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

    conn.execute(
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

    conn.execute(
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

    Ok(conn)
}

pub fn create_import_batch(
    target_month: Option<&str>,
    source_file_name: &str,
    record_count: usize,
) -> Result<String> {
    let conn = get_connection()?;

    let id = Uuid::new_v4().to_string();
    let imported_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
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