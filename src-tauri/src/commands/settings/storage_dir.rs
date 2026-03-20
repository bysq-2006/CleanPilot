use std::fs;
use std::process::Command;

use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn open_storage_directory(app: AppHandle) -> Result<(), String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;

    fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("创建应用数据目录失败: {} ({})", e, app_data_dir.display()))?;

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&app_data_dir)
            .spawn()
            .map_err(|e| format!("打开资源管理器失败: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&app_data_dir)
            .spawn()
            .map_err(|e| format!("打开 Finder 失败: {}", e))?;
    }

    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        Command::new("xdg-open")
            .arg(&app_data_dir)
            .spawn()
            .map_err(|e| format!("打开文件管理器失败: {}", e))?;
    }

    Ok(())
}

