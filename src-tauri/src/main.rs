#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
const REG_PATH: str = "SYSTEM\\CurrentControlSet\\Control\\Class\\4d36e968-e325-11ce-bfc1-08002be10318\\0000";
fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command(async)]
fn get_current_gpu() -> Result<String, String> {
  let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
  let gpu = hklm.open_subkey()?;
  Ok(gpu.get_value("Driver Desc"));
}
