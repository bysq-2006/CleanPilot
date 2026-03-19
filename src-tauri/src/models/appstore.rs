use std::sync::Mutex;

use super::config::Config;

#[derive(Default)]
pub struct AppStore {
    pub config: Mutex<Config>,
    pub chat_history: Mutex<Vec<String>>,
}

