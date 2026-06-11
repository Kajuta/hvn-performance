use calamine::{open_workbook_auto, Reader};

#[tauri::command]
pub fn inspect_excel(path: String) -> Result<String, String> {

    let workbook =
        open_workbook_auto(&path)
            .map_err(|e| e.to_string())?;

    let names = workbook.sheet_names();

    Ok(format!("{:?}", names))
}
