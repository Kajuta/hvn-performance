import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";


const feeRecordResult = document.getElementById("result-fee-record");
const feeMasterResult = document.getElementById("result-fee-master")

const unmappedList = document.getElementById("unmappedList");
const masterList = document.getElementById("masterList");

const ibowItemName = document.getElementById("ibowItemName");

const groupName = document.getElementById("groupName");
const itemType = document.getElementById("itemType");
const displayOrder = document.getElementById("displayOrder");

const newCategoryName = document.getElementById("newCategoryName");
const newCategoryGroupName = document.getElementById("newCategoryGroupName");
const newCategoryItemType = document.getElementById("newCategoryItemType");
const newCategoryDisplayOrder = document.getElementById("newCategoryDisplayOrder");

let feeCategories = [];
let currentBatchId = null;

const btnSaveMapping = document.getElementById("btnSaveMapping");
const btnSaveCategory = document.getElementById("btnSaveCategory");
const btnImportFeeRecord = document.getElementById("btn-import-fee-record");
const btnImportFeeMaster = document.getElementById("btn-import-fee-master");
const categoryName = document.getElementById("categoryName");

// Doc読み込み完了後の処理
document.addEventListener("DOMContentLoaded", async () => {

  loadCategoryOptions();
  setupTabs();

  // マッピング保存ボタンイベント
  btnSaveMapping.addEventListener("click", saveFeeItemMapping);

  // インポートボタンイベント
  btnImportFeeRecord.addEventListener("click", importVisitRecords);

  // カテゴリ保存ボタンイベント
  btnSaveCategory.addEventListener("click", saveFeeCategory);

  // カテゴリ選択時の補完
  categoryName.addEventListener("change", fillCategoryMeta);

  // 診療報酬マスタのインポート
  btnImportFeeMaster.addEventListener("click", importFeeMasterCsv)

});

// --- main function 


// タブのセットアップ
function setupTabs() {
  const tabButtons = document.querySelectorAll(".tab-button");
  const tabPanels = document.querySelectorAll(".tab-panel");

  tabButtons.forEach((button) => {

    button.addEventListener("click", () => {
      const target = button.dataset.tab;
      tabButtons.forEach((btn) => {
        btn.classList.remove("active");
      });

      tabPanels.forEach((panel) => {
        panel.classList.remove("active");
      });

      button.classList.add("active");
      const targetPanel = document.getElementById(`tab-${target}`);

      if (targetPanel) {
        targetPanel.classList.add("active");
      }
    });
  });
}

// 診療報酬マスタのインポート
async function importFeeMasterCsv() {
  try {
    const filePath = await open({
      multiple: false,
      filters: [
        {
          name: "CSV",
          extensions: ["csv"],
        },
      ],
    });

    if (!filePath) return;
    const count = await invoke("import_fee_master_csv", {
      filePath,
    });
    feeMasterResult.textContent = `${count}件取り込みました`;
  } catch (error) {
    feeMasterResult.textContent = `エラー: ${error}`;
  }
}

// 診療報酬項目マッピングの保存
async function saveFeeItemMapping() {
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

  // reload
  await loadUnmappedItems();
  await loadMasterList();
}

// ibow出力データ（訪問看護内容SeisinのXLSX）のインポート
async function importVisitRecords() {
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

    const importResult = await invoke("import_visit_records", {
      path: filePath
    });

    currentBatchId = importResult.batch_id;

    // reload
    await loadUnmappedItems();
    await loadMasterList();

  } catch (error) {
    console.log(`エラー: ${error}`)
  }

}

// 診療報酬カテゴリの保存
async function saveFeeCategory() {
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
}


// カテゴリオプションの読み込み
async function loadCategoryOptions() {
  const categories = await invoke("list_fee_categories");

  categoryName.innerHTML = "";
  feeCategories = categories;
  const option = document.createElement("option");
  option.value = "";
  option.textContent = "未選択";

  categoryName.appendChild(option);
  categories.forEach((category) => {
    const option = document.createElement("option");
    option.value = category.category_name;
    option.textContent =
      `${category.category_name} / ${category.group_name ?? ""}`;

    categoryName.appendChild(option);
  });
}

// 登録済みマスタ項目選択
function selectMasterItem(item) {
  ibowItemName.value = item.ibow_item_name;
  categoryName.value = item.category_name;
  groupName.value = item.group_name ?? "";
  itemType.value = item.item_type ?? "";
  displayOrder.value = item.display_order ?? "";
}

// カテゴリ選択時の補完
function fillCategoryMeta() {
  const selected = feeCategories.find(
    (category) => category.category_name === categoryName.value
  );

  if (!selected){
    groupName.value = "";
    itemType.value = "";
    return;
  };

  groupName.value = selected.group_name ?? "";
  itemType.value = selected.item_type ?? "";
}

// 未登録項目選択
function selectUnmappedItem(name) {
  ibowItemName.value = name;
  displayOrder.value = "";

  fillCategoryMeta();
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
    const button = document.createElement("button");

    button.type = "button";
    button.textContent =
      `${item.ibow_item_name} → ${item.category_name}`;

    button.title =
      `グループ: ${item.group_name ?? ""}\n種別: ${item.item_type ?? ""}`;

    button.addEventListener("click", () => {
      selectMasterItem(item);
    });

    masterList.appendChild(button);
  });
}

