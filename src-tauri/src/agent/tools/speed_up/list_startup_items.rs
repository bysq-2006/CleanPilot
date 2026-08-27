use std::collections::HashMap;
use std::fs;
use std::path::Path;

use tokio_util::sync::CancellationToken;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
use winreg::RegKey;
use winreg::HKEY;

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::ToolDefinition;

struct StartupItem {
    name: String,
    status: String,
    source: String,
    command: String,
}

pub fn register() -> ToolDefinition {
    ToolDefinition {
        name: "list_startup_items",
        description: "列出开机启动项，包括注册表 Run 项和启动文件夹中的程序，并尽量标注是否已禁用。适合分析开机缓慢和常驻后台的原因。不包含计划任务。",
        parameters: serde_json::json!({"type":"object","properties":{},"additionalProperties":false}),
    }
}

pub async fn call(
    _runtime: AgentRuntime,
    _payload: String,
    cancellation_token: CancellationToken,
) -> Result<String, String> {
    if cancellation_token.is_cancelled() {
        return Err("任务已取消".to_string());
    }

    let items = tokio::task::spawn_blocking(collect_startup_items)
        .await
        .map_err(|e| format!("读取开机启动项失败: {}", e))?;

    if cancellation_token.is_cancelled() {
        return Err("任务已取消".to_string());
    }

    let mut items = items?;
    items.sort_by(|left, right| {
        status_rank(&left.status)
            .cmp(&status_rank(&right.status))
            .then_with(|| left.name.to_ascii_lowercase().cmp(&right.name.to_ascii_lowercase()))
    });

    let mut lines = vec![format!("启动项数量: {}", items.len())];
    if items.is_empty() {
        lines.push("未找到开机启动项。".to_string());
        return Ok(lines.join("\n"));
    }

    for item in items {
        if cancellation_token.is_cancelled() {
            return Err("任务已取消".to_string());
        }

        lines.push(format!(
            "- 名称: {} | 状态: {} | 来源: {} | 命令: {}",
            item.name, item.status, item.source, item.command
        ));
    }

    Ok(lines.join("\n"))
}

fn collect_startup_items() -> Result<Vec<StartupItem>, String> {
    let mut approved = HashMap::new();
    load_approved(
        HKEY_CURRENT_USER,
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run",
        &mut approved,
    );
    load_approved(
        HKEY_CURRENT_USER,
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run32",
        &mut approved,
    );
    load_approved(
        HKEY_LOCAL_MACHINE,
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run",
        &mut approved,
    );
    load_approved(
        HKEY_LOCAL_MACHINE,
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run32",
        &mut approved,
    );
    load_approved(
        HKEY_CURRENT_USER,
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\StartupFolder",
        &mut approved,
    );
    load_approved(
        HKEY_LOCAL_MACHINE,
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\StartupFolder",
        &mut approved,
    );

    let mut items = Vec::new();
    collect_run_key(
        HKEY_CURRENT_USER,
        r"Software\Microsoft\Windows\CurrentVersion\Run",
        r"注册表 HKCU\...\Run",
        &approved,
        &mut items,
    );
    collect_run_key(
        HKEY_CURRENT_USER,
        r"Software\Microsoft\Windows\CurrentVersion\RunOnce",
        r"注册表 HKCU\...\RunOnce",
        &approved,
        &mut items,
    );
    collect_run_key(
        HKEY_LOCAL_MACHINE,
        r"Software\Microsoft\Windows\CurrentVersion\Run",
        r"注册表 HKLM\...\Run",
        &approved,
        &mut items,
    );
    collect_run_key(
        HKEY_LOCAL_MACHINE,
        r"Software\Microsoft\Windows\CurrentVersion\RunOnce",
        r"注册表 HKLM\...\RunOnce",
        &approved,
        &mut items,
    );
    collect_run_key(
        HKEY_LOCAL_MACHINE,
        r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Run",
        r"注册表 HKLM\WOW6432Node\...\Run",
        &approved,
        &mut items,
    );
    collect_run_key(
        HKEY_LOCAL_MACHINE,
        r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\RunOnce",
        r"注册表 HKLM\WOW6432Node\...\RunOnce",
        &approved,
        &mut items,
    );

    if let Some(appdata) = std::env::var_os("APPDATA") {
        collect_startup_folder(
            Path::new(&appdata).join(r"Microsoft\Windows\Start Menu\Programs\Startup"),
            "用户启动文件夹",
            &approved,
            &mut items,
        );
    }
    if let Some(program_data) = std::env::var_os("ProgramData") {
        collect_startup_folder(
            Path::new(&program_data).join(r"Microsoft\Windows\Start Menu\Programs\StartUp"),
            "公共启动文件夹",
            &approved,
            &mut items,
        );
    }

    Ok(items)
}

fn collect_run_key(
    hive: HKEY,
    path: &str,
    source: &str,
    approved: &HashMap<String, String>,
    items: &mut Vec<StartupItem>,
) {
    let root = RegKey::predef(hive);
    let Ok(key) = root.open_subkey(path) else {
        return;
    };

    for (name, value) in key.enum_values().filter_map(|item| item.ok()) {
        let name = name.trim().to_string();
        if name.is_empty() {
            continue;
        }

        let command = key
            .get_value::<String, _>(&name)
            .unwrap_or_else(|_| value.to_string());
        items.push(StartupItem {
            status: approved
                .get(&name)
                .cloned()
                .unwrap_or_else(|| "已启用".to_string()),
            name,
            source: source.to_string(),
            command,
        });
    }
}

fn collect_startup_folder(
    dir: impl AsRef<Path>,
    source: &str,
    approved: &HashMap<String, String>,
    items: &mut Vec<StartupItem>,
) {
    let dir = dir.as_ref();
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.filter_map(|item| item.ok()) {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name.eq_ignore_ascii_case("desktop.ini") {
            continue;
        }

        items.push(StartupItem {
            status: approved
                .get(&name)
                .cloned()
                .unwrap_or_else(|| "已启用".to_string()),
            name,
            source: source.to_string(),
            command: path.display().to_string(),
        });
    }
}

fn load_approved(hive: HKEY, path: &str, approved: &mut HashMap<String, String>) {
    let root = RegKey::predef(hive);
    let Ok(key) = root.open_subkey(path) else {
        return;
    };

    for (name, value) in key.enum_values().filter_map(|item| item.ok()) {
        approved
            .entry(name)
            .or_insert_with(|| startup_status(&value.bytes).to_string());
    }
}

fn startup_status(bytes: &[u8]) -> &'static str {
    match bytes.first() {
        Some(0x02 | 0x06) => "已启用",
        Some(0x03 | 0x07) => "已禁用",
        Some(flag) if flag & 1 == 1 => "已禁用",
        Some(_) => "已启用",
        None => "未知",
    }
}

fn status_rank(status: &str) -> u8 {
    match status {
        "已启用" => 0,
        "未知" => 1,
        _ => 2,
    }
}
