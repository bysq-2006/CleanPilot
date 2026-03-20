mod commands;
mod llm;
mod models;
mod utils;

use models::appstore::AppStore;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppStore::default())
        .setup(|app| {
            let app_handle = app.handle().clone();
            let store = app.state::<AppStore>();
            let mut config = store
                .config
                .lock()
                .map_err(|e| format!("配置锁获取失败: {}", e))?;

            config.init(&app_handle)?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::settings::set_config::get_config,
            commands::settings::set_config::save_config,
            commands::settings::storage_dir::open_storage_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
