use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use winreg::enums::*;
use winreg::RegKey;
use winreg::RegValue;
use winreg::HKEY;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupLocation {
    HkcuRun,
    HkcuRunOnce,
    HklmRun,
    HklmRunOnce,
    HklmWowRun,
    HklmWowRunOnce,
    UserFolder,
    CommonFolder,
}

impl StartupLocation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HkcuRun => "hkcu_run",
            Self::HkcuRunOnce => "hkcu_run_once",
            Self::HklmRun => "hklm_run",
            Self::HklmRunOnce => "hklm_run_once",
            Self::HklmWowRun => "hklm_wow_run",
            Self::HklmWowRunOnce => "hklm_wow_run_once",
            Self::UserFolder => "user_folder",
            Self::CommonFolder => "common_folder",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim() {
            "hkcu_run" | r"注册表 HKCU\...\Run" => Some(Self::HkcuRun),
            "hkcu_run_once" | r"注册表 HKCU\...\RunOnce" => Some(Self::HkcuRunOnce),
            "hklm_run" | r"注册表 HKLM\...\Run" => Some(Self::HklmRun),
            "hklm_run_once" | r"注册表 HKLM\...\RunOnce" => Some(Self::HklmRunOnce),
            "hklm_wow_run" | r"注册表 HKLM\WOW6432Node\...\Run" => Some(Self::HklmWowRun),
            "hklm_wow_run_once" | r"注册表 HKLM\WOW6432Node\...\RunOnce" => Some(Self::HklmWowRunOnce),
            "user_folder" | "用户启动文件夹" => Some(Self::UserFolder),
            "common_folder" | "公共启动文件夹" => Some(Self::CommonFolder),
            _ => None,
        }
    }

    pub fn source_label(self) -> &'static str {
        match self {
            Self::HkcuRun => r"注册表 HKCU\...\Run",
            Self::HkcuRunOnce => r"注册表 HKCU\...\RunOnce",
            Self::HklmRun => r"注册表 HKLM\...\Run",
            Self::HklmRunOnce => r"注册表 HKLM\...\RunOnce",
            Self::HklmWowRun => r"注册表 HKLM\WOW6432Node\...\Run",
            Self::HklmWowRunOnce => r"注册表 HKLM\WOW6432Node\...\RunOnce",
            Self::UserFolder => "用户启动文件夹",
            Self::CommonFolder => "公共启动文件夹",
        }
    }

    fn run_key(self) -> Option<(HKEY, &'static str)> {
        match self {
            Self::HkcuRun => Some((HKEY_CURRENT_USER, r"Software\Microsoft\Windows\CurrentVersion\Run")),
            Self::HkcuRunOnce => Some((HKEY_CURRENT_USER, r"Software\Microsoft\Windows\CurrentVersion\RunOnce")),
            Self::HklmRun => Some((HKEY_LOCAL_MACHINE, r"Software\Microsoft\Windows\CurrentVersion\Run")),
            Self::HklmRunOnce => Some((HKEY_LOCAL_MACHINE, r"Software\Microsoft\Windows\CurrentVersion\RunOnce")),
            Self::HklmWowRun => Some((
                HKEY_LOCAL_MACHINE,
                r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Run",
            )),
            Self::HklmWowRunOnce => Some((
                HKEY_LOCAL_MACHINE,
                r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\RunOnce",
            )),
            Self::UserFolder | Self::CommonFolder => None,
        }
    }

    fn approved_key(self) -> (HKEY, &'static str) {
        match self {
            Self::HkcuRun | Self::HkcuRunOnce => (
                HKEY_CURRENT_USER,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run",
            ),
            Self::HklmRun | Self::HklmRunOnce => (
                HKEY_LOCAL_MACHINE,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run",
            ),
            Self::HklmWowRun | Self::HklmWowRunOnce => (
                HKEY_LOCAL_MACHINE,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run32",
            ),
            Self::UserFolder => (
                HKEY_CURRENT_USER,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\StartupFolder",
            ),
            Self::CommonFolder => (
                HKEY_LOCAL_MACHINE,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\StartupFolder",
            ),
        }
    }

    fn folder_path(self) -> Option<PathBuf> {
        match self {
            Self::UserFolder => std::env::var_os("APPDATA").map(|appdata| {
                Path::new(&appdata).join(r"Microsoft\Windows\Start Menu\Programs\Startup")
            }),
            Self::CommonFolder => std::env::var_os("ProgramData").map(|program_data| {
                Path::new(&program_data).join(r"Microsoft\Windows\Start Menu\Programs\StartUp")
            }),
            _ => None,
        }
    }

    fn is_machine_wide(self) -> bool {
        matches!(
            self,
            Self::HklmRun
                | Self::HklmRunOnce
                | Self::HklmWowRun
                | Self::HklmWowRunOnce
                | Self::CommonFolder
        )
    }
}

#[derive(Debug, Clone)]
pub struct InstalledStartupItem {
    pub name: String,
    pub location: StartupLocation,
    pub command: String,
    pub enabled: bool,
}

pub fn collect_startup_items() -> Vec<InstalledStartupItem> {
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
    for location in [
        StartupLocation::HkcuRun,
        StartupLocation::HkcuRunOnce,
        StartupLocation::HklmRun,
        StartupLocation::HklmRunOnce,
        StartupLocation::HklmWowRun,
        StartupLocation::HklmWowRunOnce,
        StartupLocation::UserFolder,
        StartupLocation::CommonFolder,
    ] {
        collect_location(location, &approved, &mut items);
    }
    items
}

pub fn find_startup_item(name: &str, location: StartupLocation) -> Option<InstalledStartupItem> {
    collect_startup_items()
        .into_iter()
        .find(|item| item.location == location && item.name.eq_ignore_ascii_case(name))
}

pub fn set_startup_enabled(name: &str, location: StartupLocation, enabled: bool) -> Result<(), String> {
    let Some(item) = find_startup_item(name, location) else {
        return Err("这条开机项已经不存在".to_string());
    };

    let (hive, approved_path) = location.approved_key();
    let root = RegKey::predef(hive);
    let key = root
        .create_subkey_with_flags(approved_path, KEY_READ | KEY_WRITE)
        .map(|(key, _)| key)
        .map_err(|error| {
            if location.is_machine_wide() {
                format!("需要管理员权限才能修改这项开机项: {}", error)
            } else {
                format!("写入开机项状态失败: {}", error)
            }
        })?;

    let value = RegValue {
        bytes: approved_bytes(enabled),
        vtype: REG_BINARY,
    };
    key.set_raw_value(&item.name, &value).map_err(|error| {
        if location.is_machine_wide() {
            format!("需要管理员权限才能修改这项开机项: {}", error)
        } else {
            format!("写入开机项状态失败: {}", error)
        }
    })?;

    Ok(())
}

fn collect_location(
    location: StartupLocation,
    approved: &HashMap<String, bool>,
    items: &mut Vec<InstalledStartupItem>,
) {
    if let Some((hive, path)) = location.run_key() {
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
            items.push(InstalledStartupItem {
                enabled: approved.get(&name).copied().unwrap_or(true),
                name,
                location,
                command,
            });
        }
        return;
    }

    let Some(dir) = location.folder_path() else {
        return;
    };
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.filter_map(|item| item.ok()) {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name.eq_ignore_ascii_case("desktop.ini") {
            continue;
        }

        items.push(InstalledStartupItem {
            enabled: approved.get(&name).copied().unwrap_or(true),
            command: path.display().to_string(),
            name,
            location,
        });
    }
}

fn load_approved(hive: HKEY, path: &str, approved: &mut HashMap<String, bool>) {
    let root = RegKey::predef(hive);
    let Ok(key) = root.open_subkey(path) else {
        return;
    };

    for (name, value) in key.enum_values().filter_map(|item| item.ok()) {
        approved.entry(name).or_insert_with(|| is_approved_enabled(&value.bytes));
    }
}

fn is_approved_enabled(bytes: &[u8]) -> bool {
    match bytes.first() {
        Some(0x02 | 0x06) => true,
        Some(0x03 | 0x07) => false,
        Some(flag) if flag & 1 == 1 => false,
        Some(_) => true,
        None => true,
    }
}

fn approved_bytes(enabled: bool) -> Vec<u8> {
    let mut bytes = vec![0u8; 12];
    bytes[0] = if enabled { 0x02 } else { 0x03 };
    if !enabled {
        bytes[4..12].copy_from_slice(&windows_filetime_now());
    }
    bytes
}

fn windows_filetime_now() -> [u8; 8] {
    const UNIX_TO_WINDOWS_EPOCH: u64 = 116444736000000000;
    let unix_100ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos() / 100)
        .unwrap_or(0) as u64;
    unix_100ns.saturating_add(UNIX_TO_WINDOWS_EPOCH).to_le_bytes()
}
