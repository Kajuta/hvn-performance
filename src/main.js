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

    const importResult = await invoke("import_visit_records", {
      path: filePath
    });

    const summary = await invoke("aggregate_by_category", {
      importBatchId: importResult.batch_id
    });

    result.textContent += 
      `\n\n保存完了\nbatch_id: ${importResult.batch_id}\nrecord_count: ${importResult.record_count}`;

    result.textContent += 
      `\n\nカテゴリ別集計結果\n
      ${JSON.stringify(summary, null, 2)}`;


  } catch (error) {
    result.textContent = `エラー: ${error}`;
  }
});