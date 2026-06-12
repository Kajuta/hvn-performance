mod db;
mod ibow_import;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    db::get_connection().expect("DB init failed");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            ibow_import::inspect_excel,
            ibow_import::preview_visit_records
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
