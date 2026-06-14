use crate::db::get_connection;

use encoding_rs::SHIFT_JIS;
use rusqlite::{Connection, Transaction, params};
use serde_json::json;
use std::fs;

#[derive(Debug)]
struct FeeMasterRecord {
    fee_code: String,
    fee_name: String,
    kana_name: String,
    unit_price: i64,
    valid_from: String,
    valid_to: String,
    raw_json: String,
}

#[tauri::command]
pub fn import_fee_master_csv(file_path: String) -> Result<usize, String> {
    let mut conn = get_connection().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let records = read_fee_master_csv(&file_path)?;

    for record in &records {
        upsert_fee_master(&tx, record)?;
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(records.len())
}

fn read_fee_master_csv(file_path: &str) -> Result<Vec<FeeMasterRecord>, String> {
    let bytes = fs::read(file_path).map_err(|e| e.to_string())?;
    let (decoded, _, _) = SHIFT_JIS.decode(&bytes);
    let csv_text = decoded.to_string();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_text.as_bytes());

    let mut records = Vec::new();

    for result in reader.records() {
        let record = result.map_err(|e| e.to_string())?;

        if record.len() < 72 {
            continue;
        }

        let fee_code = record.get(2).unwrap_or("").trim().to_string();
        let fee_name = record.get(6).unwrap_or("").trim().to_string();
        let kana_name = record.get(10).unwrap_or("").trim().to_string();
        let unit_price_text = record.get(15).unwrap_or("").trim();
        let valid_from = record.get(70).unwrap_or("").trim().to_string();
        let valid_to = record.get(71).unwrap_or("").trim().to_string();

        if fee_code.is_empty() || fee_name.is_empty() {
            continue;
        }

        let unit_price = unit_price_text
            .parse::<f64>()
            .map_err(|e| format!("単価変換エラー: {} / {}", unit_price_text, e))?
            .round() as i64;

        let raw_json = json!(record.iter().collect::<Vec<_>>()).to_string();

        records.push(FeeMasterRecord {
            fee_code,
            fee_name,
            kana_name,
            unit_price,
            valid_from,
            valid_to,
            raw_json,
        });
    }

    Ok(records)
}

fn upsert_fee_master(
    tx: &Transaction,
    record: &FeeMasterRecord,
) -> Result<(), String> {
    tx.execute(
        "
        INSERT INTO fee_master (
            fee_code,
            fee_name,
            kana_name,
            unit_price,
            valid_from,
            valid_to,
            raw_json
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ON CONFLICT(fee_code) DO UPDATE SET
            fee_name = excluded.fee_name,
            kana_name = excluded.kana_name,
            unit_price = excluded.unit_price,
            valid_from = excluded.valid_from,
            valid_to = excluded.valid_to,
            raw_json = excluded.raw_json,
            updated_at = datetime('now', 'localtime')
        ",
        params![
            record.fee_code,
            record.fee_name,
            record.kana_name,
            record.unit_price,
            record.valid_from,
            record.valid_to,
            record.raw_json,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}




// ---- 以下はテスト用のコード ---

// テスト用のインターナル関数
pub fn import_fee_master_impl(
    conn: &mut Connection,
    file_path: &str
) -> Result<usize, String> {
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    

    let records = read_fee_master_csv(file_path)?;

    for record in &records {
        upsert_fee_master(&tx, record)?;
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(records.len())
}

#[cfg(test)]
mod tests {
    use crate::db;

use super::*;
    use rusqlite::{Connection};

    #[test]
    fn test_import_fee_master_csv() {
        let mut conn = Connection::open_in_memory().unwrap();

        db::init_db(&mut conn).unwrap();
        
        let result = import_fee_master_impl(
            &mut conn,
            "test_data/r_ALL20260611.csv"
        );

        println!("{:#?}", result);

        assert!(result.is_ok());
    }
}