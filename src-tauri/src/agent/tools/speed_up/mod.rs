pub mod list_processes;
pub mod list_startup_items;
pub mod system_perf;

use tokio_util::sync::CancellationToken;

async fn refreshed_system(cancellation_token: &CancellationToken) -> Result<sysinfo::System, String> {
    if cancellation_token.is_cancelled() {
        return Err("任务已取消".to_string());
    }

    let mut sys = sysinfo::System::new();
    sys.refresh_cpu();
    sys.refresh_memory();
    sys.refresh_processes();

    tokio::select! {
        _ = cancellation_token.cancelled() => return Err("任务已取消".to_string()),
        _ = tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL) => {}
    }

    sys.refresh_cpu();
    sys.refresh_memory();
    sys.refresh_processes();
    Ok(sys)
}

fn parse_optional_args<T: Default + serde::de::DeserializeOwned>(payload: &str) -> Result<T, String> {
    let payload = payload.trim();
    if payload.is_empty() {
        return Ok(T::default());
    }

    serde_json::from_str(payload).map_err(|e| format!("参数解析失败: {}", e))
}

fn normalize_windows_path(path: &str) -> String {
    path.trim_start_matches(r"\\?\")
        .replace('/', r"\")
}
