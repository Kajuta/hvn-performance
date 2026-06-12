use calamine::{open_workbook_auto, Data, Reader};
use serde::Serialize;

#[derive(Serialize)]
pub struct VisitFeeRecordPreview {
    pub user_name: String,
    pub ibow_user_id: String,
    pub fee_item_name: String,
    pub count: i32,
    pub source_row: usize,
    pub source_column: usize,
}


#[derive(Serialize)]
pub struct ExcelInspectResult {
    pub sheet_names: Vec<String>,
    pub row_count: usize,
    pub column_count: usize,
    pub headers: Vec<String>,
}

use std::collections::HashMap;

#[derive(serde::Serialize)]
pub struct FeeItemValidationResult {
    pub fee_item_name: String,
    pub converted_total: i32,
    pub excel_total: i32,
    pub diff: i32,
    pub matched: bool,
}

#[tauri::command]
pub fn validate_fee_item_totals(path: String) -> Result<Vec<FeeItemValidationResult>, String> {
    let records = preview_visit_records(path.clone())?;

    let mut workbook =
        open_workbook_auto(&path).map_err(|e| e.to_string())?;

    let sheet_name = workbook
        .sheet_names()
        .first()
        .ok_or("シートが見つかりません")?
        .to_string();

    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| e.to_string())?;

    let rows: Vec<_> = range.rows().collect();

    let header_row = rows
        .get(3)
        .ok_or("4行目の項目名が見つかりません")?;

    let total_row = rows
        .iter()
        .find(|row| is_total_row(row))
        .ok_or("合計行が見つかりません")?;

    let mut converted_map: HashMap<String, i32> = HashMap::new();

    for record in records {
        *converted_map
            .entry(record.fee_item_name)
            .or_insert(0) += record.count;
    }

    let mut results = Vec::new();

    for col_index in 2..header_row.len() {
        let fee_item_name = cell_to_string(header_row.get(col_index));

        if fee_item_name.is_empty() {
            continue;
        }

        let converted_total =
            converted_map.get(&fee_item_name).copied().unwrap_or(0);

        let excel_total =
            cell_to_i32(total_row.get(col_index));

        if converted_total == 0 && excel_total == 0 {
            continue;
        }

        results.push(FeeItemValidationResult {
            fee_item_name,
            converted_total,
            excel_total,
            diff: converted_total - excel_total,
            matched: converted_total == excel_total,
        });
    }

    Ok(results)
}

#[tauri::command]
pub fn inspect_excel(path: String) -> Result<ExcelInspectResult, String> {
    let mut workbook =
        open_workbook_auto(&path).map_err(|e| e.to_string())?;

    let sheet_name = workbook
        .sheet_names()
        .first()
        .ok_or("シートが見つかりません")?
        .to_string();

    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| e.to_string())?;

    let row_count = range.height();
    let column_count = range.width();

    // iBow出力では4行目が項目名。Rustのindexは0始まりなので3。
    let headers = range
        .rows()
        .nth(3)
        .ok_or("4行目の項目名が見つかりません")?
        .iter()
        .map(|cell| cell.to_string())
        .collect();

    Ok(ExcelInspectResult {
        sheet_names: workbook.sheet_names().to_vec(),
        row_count,
        column_count,
        headers,
    })
}


#[tauri::command]
pub fn preview_visit_records(path: String) -> Result<Vec<VisitFeeRecordPreview>, String> {
    let mut workbook =
        open_workbook_auto(&path).map_err(|e| e.to_string())?;

    let sheet_name = workbook
        .sheet_names()
        .first()
        .ok_or("シートが見つかりません")?
        .to_string();

    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| e.to_string())?;

    let rows: Vec<_> = range.rows().collect();

    let header_row = rows
        .get(3)
        .ok_or("4行目の項目名が見つかりません")?;

    let mut records = Vec::new();

    for (row_index, row) in rows.iter().enumerate().skip(4) {
        let user_name = cell_to_string(row.get(0));
        let ibow_user_id = cell_to_string(row.get(1));

        if user_name.is_empty() || is_total_row(row) {
            continue;
        }

        for col_index in 2..header_row.len() {
            let fee_item_name = cell_to_string(header_row.get(col_index));

            if fee_item_name.is_empty() {
                continue;
            }

            let count = cell_to_i32(row.get(col_index));

            if count == 0 {
                continue;
            }

            records.push(VisitFeeRecordPreview {
                user_name: user_name.clone(),
                ibow_user_id: ibow_user_id.clone(),
                fee_item_name,
                count,
                source_row: row_index + 1,
                source_column: col_index + 1,
            });
        }
    }

    Ok(records)
}

fn cell_to_string(cell: Option<&Data>) -> String {
    match cell {
        Some(Data::String(s)) => s.trim().to_string(),
        Some(Data::Float(n)) => {
            if n.fract() == 0.0 {
                format!("{}", *n as i64)
            } else {
                n.to_string()
            }
        }
        Some(Data::Int(n)) => n.to_string(),
        Some(Data::Bool(b)) => b.to_string(),
        _ => String::new(),
    }
}

fn cell_to_i32(cell: Option<&Data>) -> i32 {
    match cell {
        Some(Data::Int(n)) => *n as i32,
        Some(Data::Float(n)) => *n as i32,
        Some(Data::String(s)) => s.trim().parse::<i32>().unwrap_or(0),
        _ => 0,
    }
}

fn normnalize_label(value: &str) -> String {
    value
        .replace(" ", "")
        .replace("　", "")
        .trim()
        .to_string()
}

fn is_total_row(row: &[Data]) -> bool {
    let label = normnalize_label(&cell_to_string(row.get(0)));
    label == "合計"
}