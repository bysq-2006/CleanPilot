use std::os::windows::process::CommandExt;
use std::process::Command;

use tokio_util::sync::CancellationToken;

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::ToolDefinition;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[repr(C)]
struct SystemPowerStatus {
    ac_line_status: u8,
    battery_flag: u8,
    battery_life_percent: u8,
    system_status_flag: u8,
    battery_life_time: u32,
    battery_full_life_time: u32,
}

#[link(name = "kernel32")]
extern "system" {
    fn GetSystemPowerStatus(status: *mut SystemPowerStatus) -> i32;
}

pub fn register() -> ToolDefinition {
    ToolDefinition {
        name: "get_system_perf",
        description: "获取当前电脑的整体运行情况，包括 CPU 占用、内存、磁盘空间、电源计划和设备类型。适合在诊断卡顿时先建立全局认知。",
        parameters: serde_json::json!({"type":"object","properties":{},"additionalProperties":false}),
    }
}

pub async fn call(
    _runtime: AgentRuntime,
    _payload: String,
    cancellation_token: CancellationToken,
) -> Result<String, String> {
    let sys = super::refreshed_system(&cancellation_token).await?;
    if cancellation_token.is_cancelled() {
        return Err("任务已取消".to_string());
    }

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let logical_cpus = sys.cpus().len();
    let physical_cores = sys.physical_core_count().unwrap_or(0);
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let available_memory = sys.available_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();

    let mut lines = vec![
        format!("系统: {}", sysinfo::System::long_os_version().unwrap_or_else(|| "未知".to_string())),
        format!("设备类型: {}", device_kind()),
        format!("电源计划: {}", active_power_plan()),
        format!("CPU 占用: {:.1}%", cpu_usage),
        format!("逻辑处理器: {}", logical_cpus),
        format!("物理核心: {}", physical_cores),
        format!("内存总量: {} 字节", total_memory),
        format!("内存已用: {} 字节", used_memory),
        format!("内存可用: {} 字节", available_memory),
        format!("交换总量: {} 字节", total_swap),
        format!("交换已用: {} 字节", used_swap),
        "磁盘:".to_string(),
    ];

    let disks = sysinfo::Disks::new_with_refreshed_list();
    if disks.iter().next().is_none() {
        lines.push("- 未找到磁盘信息".to_string());
    } else {
        for disk in disks.iter() {
            if cancellation_token.is_cancelled() {
                return Err("任务已取消".to_string());
            }

            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space.saturating_sub(available_space);
            lines.push(format!(
                "- 挂载点: {} | 文件系统: {} | 总容量: {} 字节 | 已用: {} 字节 | 可用: {} 字节",
                super::normalize_windows_path(&disk.mount_point().display().to_string()),
                disk.file_system().to_string_lossy(),
                total_space,
                used_space,
                available_space
            ));
        }
    }

    Ok(lines.join("\n"))
}

fn device_kind() -> String {
    let mut status = SystemPowerStatus {
        ac_line_status: 0,
        battery_flag: 0,
        battery_life_percent: 0,
        system_status_flag: 0,
        battery_life_time: 0,
        battery_full_life_time: 0,
    };

    let ok = unsafe { GetSystemPowerStatus(&mut status) } != 0;
    if !ok {
        return "未知".to_string();
    }

    if status.battery_flag & 0x80 != 0 {
        "台式机".to_string()
    } else {
        "笔记本".to_string()
    }
}

fn active_power_plan() -> String {
    let output = Command::new("powercfg")
        .args(["/getactivescheme"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    let Ok(output) = output else {
        return "未知".to_string();
    };
    if !output.status.success() {
        return "未知".to_string();
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let guid = extract_guid(&text).unwrap_or_default();
    if guid.is_empty() {
        return "未知".to_string();
    }

    let name = match guid.to_ascii_lowercase().as_str() {
        "381b4222-f694-41f0-9685-ff5bb260df2e" => "平衡",
        "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c" => "高性能",
        "a1841308-3541-4fab-bc81-f71556f20b4a" => "节能",
        "e9a42b02-d5df-448d-aa00-03f14749eb61" => "卓越性能",
        _ => "自定义",
    };

    format!("{name} ({guid})")
}

fn extract_guid(text: &str) -> Option<String> {
    let start = text.find(|ch: char| ch.is_ascii_hexdigit())?;
    let slice = &text[start..];
    let guid: String = slice
        .chars()
        .take_while(|ch| ch.is_ascii_hexdigit() || *ch == '-')
        .collect();
    if guid.len() >= 36 {
        Some(guid[..36].to_string())
    } else {
        None
    }
}
