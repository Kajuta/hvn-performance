use rusqlite::{Connection, Result};

pub fn get_connection() -> Result<Connection> {
    let conn = Connection::open("hvn-performance.db")?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS import_batches (
            id TEXT PRIMARY KEY,
            source_file_name TEXT NOT NULL,
            imported_at TEXT NOT NULL
        )
        ",
        [],
    )?;

    Ok(conn)
}
