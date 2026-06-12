import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

const btnImport = document.getElementById("btnImport");
const result = document.getElementById("result");

btnImport.addEventListener("click", async () => {
  try {
    const filePath = await open({
      multiple: false,
      filters: [
        {
          name: "Excel",
          extensions: ["xlsx", "xlsm", "xls"]
        }
      ]
    });

    if (!filePath) return;

    const data = await invoke("inspect_excel", {
      path: filePath
    });

    const records = await invoke("preview_visit_records", {
      path: filePath
    });

    const validation = await invoke("validate_fee_item_totals", {
      path: filePath
    });

    result.textContent = JSON.stringify(validation, null, 2);

  } catch (error) {
    result.textContent = `エラー: ${error}`;
  }
});