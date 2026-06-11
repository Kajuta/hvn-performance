
mod db;
mod ibow_import;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {

    db::get_connection().expect("DB init failed");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            ibow_import::inspect_excel
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

