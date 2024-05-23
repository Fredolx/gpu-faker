#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{backtrace::Backtrace, error::Error, fs, path::PathBuf};

use directories::ProjectDirs;
use winreg::{
    enums::{HKEY_LOCAL_MACHINE, KEY_READ, KEY_SET_VALUE},
    RegKey,
};
const REG_PATH: &str =
    "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}\\0000";
const REG_VALUE_NAME: &str = "DriverDesc";
fn main() -> Result<(), String> {
    create_data_dir()?;
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_current_gpu,
            apply_desired_gpu,
            restore,
            backup_exists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

fn get_base_project_path() -> Result<ProjectDirs, String> {
    Ok(
        directories::ProjectDirs::from("dev", "fredol", "gpu-faker")
            .ok_or("Path does not exist")?,
    )
}

fn get_backup_path() -> Result<PathBuf, String> {
    Ok(get_base_project_path()?.data_dir().join("backup.txt"))
}

fn create_data_dir() -> Result<(), String> {
    let project_dirs = get_base_project_path()?;
    let path = project_dirs.data_dir();
    if !path.exists() {
        fs::create_dir_all(path).map_err(map_error)?;
    }
    Ok(())
}

fn map_error<E: Error>(e: E) -> String {
    return format!("{}\n{}", Backtrace::force_capture(), e.to_string());
}

fn get_key(write: bool) -> Result<RegKey, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let access = match write {
        true => KEY_SET_VALUE,
        false => KEY_READ,
    };
    return hklm
        .open_subkey_with_flags(REG_PATH, access)
        .map_err(map_error);
}

#[tauri::command(async)]
fn get_current_gpu() -> Result<String, String> {
    let key = get_key(false)?;
    return key.get_value(REG_VALUE_NAME).map_err(map_error);
}

#[tauri::command(async)]
fn backup_exists() -> Result<bool, String> {
    let path = get_backup_path()?;
    Ok(path.exists())
}

fn backup(gpu: &String) -> Result<(), String> {
    let path = get_backup_path()?;
    if !path.exists() {
        fs::write(path, gpu).map_err(map_error)?;
    }
    Ok(())
}

#[tauri::command(async)]
fn restore() -> Result<(), String> {
    let path = get_backup_path()?;
    let gpu = fs::read_to_string(&path).map_err(map_error)?;
    apply_desired_gpu(gpu, None)?;
    fs::remove_file(path).map_err(map_error)?;
    Ok(())
}

#[tauri::command(async)]
fn apply_desired_gpu(gpu: String, old_gpu: Option<String>) -> Result<(), String> {
    if let Some(old_gpu) = old_gpu {
      backup(&old_gpu)?;
    }
    let key = get_key(true)?;
    return key.set_value(REG_VALUE_NAME, &gpu).map_err(map_error);
}
