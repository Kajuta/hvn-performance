mod db;
mod ibow_import;
mod aggregate;
mod fee_master;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut conn = db::get_connection()
        .expect("DB connection failed");

    db::init_db(&mut conn)
        .expect("DB init failed");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            ibow_import::inspect_excel,
            ibow_import::preview_visit_records,
            ibow_import::validate_fee_item_totals,
            ibow_import::import_visit_records,
            aggregate::aggregate_by_category,
            aggregate::find_unmapped_fee_items,
            aggregate::save_fee_item_mapping,
            aggregate::list_fee_item_master,
            aggregate::list_fee_categories,
            aggregate::save_fee_category,
            fee_master::import_fee_master_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
