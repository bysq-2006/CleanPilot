mod llm;

use std::collections::HashMap;

#[tauri::command]
async fn chat_with_llm(
    provider: String,
    prompt: String,
    config: HashMap<String, String>,
) -> Result<llm::types::LlmResponse, String> {
    llm::chat(provider, prompt, config).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![chat_with_llm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
