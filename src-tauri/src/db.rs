use rusqlite::{Connection, Result};

pub fn get_connection() -> Result<Connection> {
    let conn = Connection::open("../hvn-performance.db")?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS import_batches (
            id TEXT PRIMARY KEY,
            source_file_name TEXT NOT NULL,
            source_path TEXT NOT NULL,
            sheet_name TEXT,
            row_count INTEGER,
            column_count INTEGER,
            imported_at TEXT NOT NULL
        )
        ",
        [],
    )?;

    Ok(conn)
}

use chrono::Local;
use uuid::Uuid;

pub fn create_import_batch(
    source_file_name: &str,
    source_path: &str,
    sheet_name: &str,
    row_count: usize,
    column_count: usize,
) -> Result<String> {
    let conn = get_connection()?;

    let id = Uuid::new_v4().to_string();
    let imported_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "
        INSERT INTO import_batches (
            id,
            source_file_name,
            source_path,
            sheet_name,
            row_count,
            column_count,
            imported_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ",
        (
            &id,
            source_file_name,
            source_path,
            sheet_name,
            row_count as i64,
            column_count as i64,
            imported_at,
        ),
    )?;

    Ok(id)
}