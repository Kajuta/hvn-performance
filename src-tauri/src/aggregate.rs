use rusqlite::Result;
use crate::db;
use serde::{Deserialize, Serialize};

#[derive(serde::Serialize)]
pub struct UnmappedFeeItem {
    pub ibow_item_name: String,
}

#[derive(Deserialize)]
pub struct FeeItemMappingInput {
    pub ibow_item_name: String,
    pub category_name: String,
    pub group_name: Option<String>,
    pub item_type: Option<String>,
    pub display_order: Option<i32>,
}


#[derive(Serialize)]
pub struct CategorySummary {
    pub category_name: String,
    pub group_name: Option<String>,
    pub item_type: Option<String>,
    pub total_count: i32,
}


#[derive(serde::Serialize)]
pub struct FeeItemMasterRow {
    pub id: i64,
    pub ibow_item_name: String,
    pub category_name: String,
    pub group_name: Option<String>,
    pub item_type: Option<String>,
    pub display_order: Option<i32>,
    pub is_active: i32,
}

#[derive(serde::Serialize)]
pub struct FeeCategoryRow {
    pub id: i64,
    pub category_name: String,
    pub group_name: Option<String>,
    pub item_type: Option<String>,
    pub display_order: Option<i32>,
    pub is_active: i32,
}

#[derive(serde::Deserialize)]
pub struct FeeCategoryInput {
    pub category_name: String,
    pub group_name: Option<String>,
    pub item_type: Option<String>,
    pub display_order: Option<i32>,
}

// カテゴリを保存する
#[tauri::command]
pub fn save_fee_category(
    input: FeeCategoryInput,
) -> Result<(), String> {
    let conn = db::get_connection()
        .map_err(|e| e.to_string())?;

    conn.execute(
        "
        INSERT INTO fee_category_master (
            category_name,
            group_name,
            item_type,
            display_order,
            is_active
        )
        VALUES (?1, ?2, ?3, ?4, 1)
        ON CONFLICT(category_name)
        DO UPDATE SET
            group_name = excluded.group_name,
            item_type = excluded.item_type,
            display_order = excluded.display_order,
            is_active = 1
        ",
        (
            input.category_name,
            input.group_name,
            input.item_type,
            input.display_order,
        ),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

// カテゴリの一覧を取得する
#[tauri::command]
pub fn list_fee_categories() -> Result<Vec<FeeCategoryRow>, String> {
    let conn = db::get_connection()
        .map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "
        SELECT
            id,
            category_name,
            group_name,
            item_type,
            display_order,
            is_active
        FROM fee_category_master
        WHERE is_active = 1
        ORDER BY display_order, category_name
        "
    )
    .map_err(|e| e.to_string())?;

    let rows = stmt.query_map([], |row| {
        Ok(FeeCategoryRow {
            id: row.get(0)?,
            category_name: row.get(1)?,
            group_name: row.get(2)?,
            item_type: row.get(3)?,
            display_order: row.get(4)?,
            is_active: row.get(5)?,
        })
    })
    .map_err(|e| e.to_string())?;

    let mut items = Vec::new();

    for row in rows {
        items.push(row.map_err(|e| e.to_string())?);
    }

    Ok(items)
}


#[tauri::command]
pub fn list_fee_item_master() -> Result<Vec<FeeItemMasterRow>, String> {
    let conn = db::get_connection()
        .map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "
        SELECT
            id,
            ibow_item_name,
            category_name,
            group_name,
            item_type,
            display_order,
            is_active
        FROM fee_item_master
        ORDER BY
            display_order,
            ibow_item_name
        "
    )
    .map_err(|e| e.to_string())?;

    let rows = stmt.query_map([], |row| {
        Ok(FeeItemMasterRow {
            id: row.get(0)?,
            ibow_item_name: row.get(1)?,
            category_name: row.get(2)?,
            group_name: row.get(3)?,
            item_type: row.get(4)?,
            display_order: row.get(5)?,
            is_active: row.get(6)?,
        })
    })
    .map_err(|e| e.to_string())?;

    let mut items = Vec::new();

    for row in rows {
        items.push(row.map_err(|e| e.to_string())?);
    }

    Ok(items)
}


// 分類を保存する
#[tauri::command]
pub fn save_fee_item_mapping(
    input: FeeItemMappingInput,
) -> Result<(), String> {
    let conn = db::get_connection()
        .map_err(|e| e.to_string())?;

    conn.execute(
        "
        INSERT INTO fee_item_master (
            ibow_item_name,
            category_name,
            group_name,
            item_type,
            display_order,
            is_active
        )
        VALUES (?1, ?2, ?3, ?4, ?5, 1)
        ON CONFLICT(ibow_item_name)
        DO UPDATE SET
            category_name = excluded.category_name,
            group_name = excluded.group_name,
            item_type = excluded.item_type,
            display_order = excluded.display_order,
            is_active = 1
        ",
        (
            input.ibow_item_name,
            input.category_name,
            input.group_name,
            input.item_type,
            input.display_order,
        ),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}


// 未分類の項目を取得する
#[tauri::command]
pub fn find_unmapped_fee_items(
    import_batch_id: String,
) -> Result<Vec<UnmappedFeeItem>, String> {
    let conn = db::get_connection()
        .map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "
        SELECT DISTINCT
            r.fee_item_name
        FROM visit_fee_records r
        LEFT JOIN fee_item_master m
            ON r.fee_item_name = m.ibow_item_name
        WHERE r.import_batch_id = ?1
          AND m.id IS NULL
        ORDER BY r.fee_item_name
        "
    )
    .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([import_batch_id], |row| {
            Ok(UnmappedFeeItem {
                ibow_item_name: row.get(0)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut items = Vec::new();

    for row in rows {
        items.push(row.map_err(|e| e.to_string())?);
    }

    Ok(items)
}

// 項目別に集計する
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