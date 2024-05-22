#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use winreg::{enums::{HKEY_LOCAL_MACHINE, KEY_READ, KEY_SET_VALUE}, RegKey};
const REG_PATH: &str = "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}\\0000";
const REG_VALUE_NAME: &str = "DriverDesc";
fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![get_current_gpu, apply_desired_gpu])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn get_key(write: bool) -> Result<RegKey, String> {
  let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
  let access = match write {
      true => KEY_SET_VALUE,
      false => KEY_READ
  };
  return hklm.open_subkey_with_flags(REG_PATH, access).map_err(|e| e.to_string());
}

#[tauri::command(async)]
fn get_current_gpu() -> Result<String, String> {
  let key = get_key(false)?;
  return key.get_value(REG_VALUE_NAME).map_err(|e| e.to_string());
}

#[tauri::command(async)]
fn apply_desired_gpu(gpu: String) -> Result<(), String> {
  let key = get_key(true)?;
  return key.set_value(REG_VALUE_NAME, &gpu).map_err(|e| e.to_string());
}

