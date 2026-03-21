use std::time::Duration;
use tokio::time::sleep;

pub fn start_agent_loop() {
    tauri::async_runtime::spawn(async move {
        run_loop().await;
    });
}

async fn run_loop() {
    loop {
        sleep(Duration::from_millis(100)).await;
    }
}
