mod agent;
mod commands;
mod manager;
mod models;
mod utils;

use models::appstore::AppStore;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            let store = AppStore::new(&app_handle)?;
            app.manage(store);

            let store = app.state::<AppStore>();
            let mut config = store
                .config
                .lock()
                .map_err(|e| format!("配置锁获取失败: {}", e))?;

            config.init(&app_handle)?;
            drop(config);

            store.init_agent()?;
            let agent = store
                .agent
                .lock()
                .map_err(|e| format!("Agent 锁获取失败: {}", e))?;
            agent
                .as_ref()
                .ok_or_else(|| "Agent 初始化失败".to_string())?
                .start();

            store.manager.start_auto_save(app_handle.clone());

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::agent::chat,
            commands::agent::get_history,
            commands::manager_context::list_history_records,
            commands::manager_context::get_current_scene,
            commands::manager_context::create_history_context,
            commands::manager_context::restore_history_context,
            commands::manager_context::switch_current_scene,
            commands::agent::debug_print_history,
            commands::settings::set_config::get_config,
            commands::settings::set_config::save_config,
            commands::settings::storage_dir::open_storage_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
