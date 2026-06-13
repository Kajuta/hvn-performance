mod db;
mod ibow_import;
mod aggregate;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    db::get_connection().expect("DB init failed");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            ibow_import::inspect_excel,
            ibow_import::preview_visit_records,
            ibow_import::validate_fee_item_totals,
            ibow_import::import_visit_records,
            aggregate::aggregate_by_category,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
