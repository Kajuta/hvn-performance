import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

const btnImport = document.getElementById("btnImport");
const result = document.getElementById("result");

const unmappedList = document.getElementById("unmappedList");
const masterList = document.getElementById("masterList");

const ibowItemName = document.getElementById("ibowItemName");
const categoryName = document.getElementById("categoryName");
const groupName = document.getElementById("groupName");
const itemType = document.getElementById("itemType");
const displayOrder = document.getElementById("displayOrder");

const btnSaveMapping = document.getElementById("btnSaveMapping");

const newCategoryName = document.getElementById("newCategoryName");
const newCategoryGroupName = document.getElementById("newCategoryGroupName");
const newCategoryItemType = document.getElementById("newCategoryItemType");
const newCategoryDisplayOrder = document.getElementById("newCategoryDisplayOrder");
const btnSaveCategory = document.getElementById("btnSaveCategory");

let feeCategories = [];
let currentBatchId = null;


// カテゴリオプションの読み込み
async function loadCategoryOptions() {
  const categories = await invoke("list_fee_categories");

  categoryName.innerHTML = "";

  categories.forEach((category) => {
    const option = document.createElement("option");
    option.value = category.category_name;
    option.textContent =
      `${category.category_name} / ${category.group_name ?? ""}`;

    categoryName.appendChild(option);
  });
}

// 未マッピング項目選択
function selectUnmappedItem(name) {
  ibowItemName.value = name;
  categoryName.value = "";
  groupName.value = "";
  itemType.value = "";
  displayOrder.value = "";
}

async function loadUnmappedItems() {
  if (!currentBatchId) return;

  const items = await invoke("find_unmapped_fee_items", {
    importBatchId: currentBatchId
  });

  unmappedList.innerHTML = "";

  items.forEach((item) => {
    const button = document.createElement("button");
    button.textContent = item.ibow_item_name;
    button.addEventListener("click", () => {
      selectUnmappedItem(item.ibow_item_name);
    });

    unmappedList.appendChild(button);
  });
}

// マスタリストの読み込み
async function loadMasterList() {
  const items = await invoke("list_fee_item_master");

  masterList.innerHTML = "";

  items.forEach((item) => {
    const div = document.createElement("div");

    div.textContent =
      `${item.ibow_item_name} → ${item.category_name} / ${item.group_name ?? ""} / ${item.item_type ?? ""}`;

    masterList.appendChild(div);
  });
}

// マッピング保存ボタンイベント
btnSaveMapping.addEventListener("click", async () => {
  await invoke("save_fee_item_mapping", {
    input: {
      ibow_item_name: ibowItemName.value,
      category_name: categoryName.value,
      group_name: groupName.value || null,
      item_type: itemType.value || null,
      display_order: displayOrder.value
        ? Number(displayOrder.value)
        : null
    }
  });

  await loadUnmappedItems();
  await loadMasterList();
});

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

    currentBatchId = importResult.batch_id;

    await loadUnmappedItems();
    await loadMasterList();


  } catch (error) {
    result.textContent = `エラー: ${error}`;
  }
});

// カテゴリ保存ボタンイベント
btnSaveCategory.addEventListener("click", async () => {
  await invoke("save_fee_category", {
    input: {
      category_name: newCategoryName.value,
      group_name: newCategoryGroupName.value || null,
      item_type: newCategoryItemType.value || null,
      display_order: newCategoryDisplayOrder.value
        ? Number(newCategoryDisplayOrder.value)
        : null
    }
  });

  newCategoryName.value = "";
  newCategoryGroupName.value = "";
  newCategoryItemType.value = "";
  newCategoryDisplayOrder.value = "";

  await loadCategoryOptions();
});

// カテゴリ選択時の補完
categoryName.addEventListener("change", () => {
  const selected = feeCategories.find(
    (category) => category.category_name === categoryName.value
  );

  if (!selected) return;

  groupName.value = selected.group_name ?? "";
  itemType.value = selected.item_type ?? "";
});

