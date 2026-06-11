import { invoke } from "@tauri-apps/api/core";

document
  .getElementById("btnTest")
  .addEventListener("click", async () => {

    const result =
      await invoke(
        "inspect_excel",
        {
          path: "Seisin.xlsx"
        }
      );

    document.getElementById("result")
      .textContent = result;
});