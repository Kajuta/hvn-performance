use rusqlite::Result;
use serde::Serialize;

use crate::db;

#[derive(Serialize)]
pub struct CategorySummary {
    pub category_name: String,
    pub group_name: Option<String>,
    pub item_type: Option<String>,
    pub total_count: i32,
}

#[tauri::command]
pub fn aggregate_by_category(import_batch_id: String) -> Result<Vec<CategorySummary>, String> {
    let conn = db::get_connection()
        .map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "
        SELECT
            m.category_name,
            m.group_name,
            m.item_type,
            SUM(r.count) AS total_count
        FROM visit_fee_records r
        INNER JOIN fee_item_master m
            ON r.fee_item_name = m.ibow_item_name
        WHERE r.import_batch_id = ?1
          AND m.is_active = 1
        GROUP BY
            m.category_name,
            m.group_name,
            m.item_type
        ORDER BY
            MIN(m.display_order),
            m.category_name
        "
    )
    .map_err(|e| e.to_string())?;

    let rows = stmt.query_map([import_batch_id], |row| {
        Ok(CategorySummary {
            category_name: row.get(0)?,
            group_name: row.get(1)?,
            item_type: row.get(2)?,
            total_count: row.get(3)?,
        })
    })
    .map_err(|e| e.to_string())?;

    let mut results = Vec::new();

    for row in rows {
        results.push(row.map_err(|e| e.to_string())?);
    }

    Ok(results)
}