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

        if user_name.is_empty() || user_name == "合計" {
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