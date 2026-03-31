use std::sync::Arc;

use tauri::AppHandle;

pub mod history;

use self::history::HistoryManager;

/// 管理模块入口。
#[derive(Debug, Clone, Default)]
pub struct ManagerModule {
    pub history: Arc<HistoryManager>,
}

impl ManagerModule {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        Ok(Self {
            history: Arc::new(HistoryManager::new(app)?),
        })
    }
}
