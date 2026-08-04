use std::fs;
use std::path::Path;
use std::process::Command;

use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn open_storage_directory(app: AppHandle) -> Result<(), String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;

    open_directory(&path)
}

#[tauri::command]
pub fn open_log_directory(app: AppHandle) -> Result<(), String> {
    let path = app
        .path()
        .app_log_dir()
        .map_err(|e| format!("无法获取日志目录: {}", e))?;

    open_directory(&path)
}

fn open_directory(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path)
        .map_err(|e| format!("创建目录失败: {} ({})", e, path.display()))?;

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(path)
            .spawn()
            .map_err(|e| format!("打开资源管理器失败: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .spawn()
            .map_err(|e| format!("打开 Finder 失败: {}", e))?;
    }

    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map_err(|e| format!("打开文件管理器失败: {}", e))?;
    }

    Ok(())
}

