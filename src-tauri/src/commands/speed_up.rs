use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::models::appstore::AppStore;
use crate::utils::process_live::{end_matching_processes, find_running_matches};
use crate::utils::startup::{find_startup_item, set_startup_enabled, StartupLocation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedUpProcessItem {
    pub name: String,
    pub path: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedUpStartupItem {
    pub name: String,
    pub location: String,
    pub command: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpeedUpChecklist {
    #[serde(default)]
    pub processes: Vec<SpeedUpProcessItem>,
    #[serde(default)]
    pub startup_items: Vec<SpeedUpStartupItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LiveSpeedUpProcessItem {
    pub name: String,
    pub path: String,
    pub reason: String,
    pub running: bool,
    pub instance_count: u32,
    pub memory: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct LiveSpeedUpStartupItem {
    pub name: String,
    pub location: String,
    pub command: String,
    pub reason: String,
    pub enabled: bool,
    pub found: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct LiveSpeedUpItems {
    pub processes: Vec<LiveSpeedUpProcessItem>,
    pub startup_items: Vec<LiveSpeedUpStartupItem>,
}

fn read_checklist(app: &AppHandle, record_path: &str) -> Result<SpeedUpChecklist, String> {
    let store = app.state::<AppStore>();
    let record = store.manager.storage_box.read_record(record_path.to_string())?;
    if record.task_type != "speed_up" {
        return Err("当前任务不是电脑变快任务".to_string());
    }

    serde_json::from_value(record.content).map_err(|e| format!("解析 speed_up 清单失败: {}", e))
}

fn require_process(checklist: &SpeedUpChecklist, name: &str, path: &str) -> Result<(), String> {
    if checklist
        .processes
        .iter()
        .any(|item| item.name == name && item.path == path)
    {
        Ok(())
    } else {
        Err("目标进程不属于当前任务".to_string())
    }
}

fn require_startup(checklist: &SpeedUpChecklist, name: &str, location: &str) -> Result<(), String> {
    if checklist
        .startup_items
        .iter()
        .any(|item| item.name == name && item.location == location)
    {
        Ok(())
    } else {
        Err("目标开机项不属于当前任务".to_string())
    }
}

#[tauri::command]
pub fn get_speed_up_items(app: AppHandle, path: String) -> Result<LiveSpeedUpItems, String> {
    let checklist = read_checklist(&app, &path)?;

    let processes = checklist
        .processes
        .into_iter()
        .map(|item| {
            let live = find_running_matches(&item.name, &item.path);
            LiveSpeedUpProcessItem {
                name: item.name,
                path: item.path,
                reason: item.reason,
                running: live.instance_count > 0,
                instance_count: live.instance_count,
                memory: live.memory,
            }
        })
        .collect();

    let startup_items = checklist
        .startup_items
        .into_iter()
        .map(|item| {
            let location = StartupLocation::parse(&item.location);
            let live = location.and_then(|location| find_startup_item(&item.name, location));
            LiveSpeedUpStartupItem {
                name: item.name,
                location: item.location,
                command: item.command,
                reason: item.reason,
                enabled: live.as_ref().map(|value| value.enabled).unwrap_or(false),
                found: live.is_some(),
            }
        })
        .collect();

    Ok(LiveSpeedUpItems {
        processes,
        startup_items,
    })
}

#[tauri::command]
pub fn set_speed_up_startup(
    app: AppHandle,
    record_path: String,
    name: String,
    location: String,
    enabled: bool,
) -> Result<LiveSpeedUpStartupItem, String> {
    let checklist = read_checklist(&app, &record_path)?;
    require_startup(&checklist, &name, &location)?;

    let parsed = StartupLocation::parse(&location).ok_or_else(|| format!("未知的开机项位置: {}", location))?;
    set_startup_enabled(&name, parsed, enabled)?;

    let live = find_startup_item(&name, parsed);
    let command = checklist
        .startup_items
        .iter()
        .find(|item| item.name == name && item.location == location)
        .map(|item| item.command.clone())
        .unwrap_or_default();
    let reason = checklist
        .startup_items
        .iter()
        .find(|item| item.name == name && item.location == location)
        .map(|item| item.reason.clone())
        .unwrap_or_default();

    Ok(LiveSpeedUpStartupItem {
        name,
        location,
        command,
        reason,
        enabled: live.as_ref().map(|value| value.enabled).unwrap_or(enabled),
        found: live.is_some(),
    })
}

#[tauri::command]
pub fn end_speed_up_process(
    app: AppHandle,
    record_path: String,
    name: String,
    path: String,
) -> Result<LiveSpeedUpProcessItem, String> {
    let checklist = read_checklist(&app, &record_path)?;
    require_process(&checklist, &name, &path)?;

    let ended = end_matching_processes(&name, &path)?;
    let live = find_running_matches(&name, &path);
    if ended == 0 && live.instance_count == 0 {
        // already gone
    } else if ended == 0 {
        return Err("结束进程失败".to_string());
    }

    let reason = checklist
        .processes
        .iter()
        .find(|item| item.name == name && item.path == path)
        .map(|item| item.reason.clone())
        .unwrap_or_default();

    Ok(LiveSpeedUpProcessItem {
        name,
        path,
        reason,
        running: live.instance_count > 0,
        instance_count: live.instance_count,
        memory: live.memory,
    })
}
