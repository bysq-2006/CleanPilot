use sysinfo::{Process, System};

pub fn normalize_windows_path(path: &str) -> String {
    path.trim_start_matches(r"\\?\")
        .replace('/', r"\")
        .trim()
        .trim_end_matches(['\\', '/'])
        .to_ascii_lowercase()
}

pub fn process_category(name: &str) -> &'static str {
    let lower = name.to_ascii_lowercase();
    let stem = lower.trim_end_matches(".exe");

    const CRITICAL: &[&str] = &[
        "system",
        "registry",
        "smss",
        "csrss",
        "wininit",
        "services",
        "lsass",
        "svchost",
        "winlogon",
        "dwm",
        "secure system",
        "memory compression",
        "idle",
        "system idle process",
        "fontdrvhost",
        "lsaiso",
        "conhost",
        "sihost",
        "runtimebroker",
        "searchhost",
        "shellexperiencehost",
        "textinputhost",
        "ctfmon",
        "taskhostw",
        "dllhost",
        "explorer",
    ];
    const SECURITY: &[&str] = &[
        "msmpeng",
        "nissrv",
        "securityhealthservice",
        "securityhealthsystray",
        "smartscreen",
        "mpdefendercoreservice",
        "securityhealthhost",
    ];

    if CRITICAL.iter().any(|item| stem == *item) {
        "系统关键"
    } else if SECURITY.iter().any(|item| stem == *item) {
        "安全软件"
    } else {
        "普通"
    }
}

pub fn is_protected_process(name: &str) -> bool {
    process_category(name) != "普通"
}

fn is_generic_name(name: &str) -> bool {
    let stem = name.to_ascii_lowercase();
    let stem = stem.trim_end_matches(".exe");
    matches!(
        stem,
        "svchost"
            | "dllhost"
            | "rundll32"
            | "conhost"
            | "runtimebroker"
            | "taskhostw"
            | "wmiprvse"
            | "explorer"
            | "system"
    )
}

fn has_usable_path(path: &str) -> bool {
    let path = path.trim();
    !path.is_empty() && path != "未知"
}

fn same_process_name(left: &str, right: &str) -> bool {
    let normalize = |value: &str| value.trim().trim_end_matches(".exe").to_ascii_lowercase();
    normalize(left) == normalize(right)
}

fn process_exe_path(process: &Process) -> String {
    process
        .exe()
        .map(|path| normalize_windows_path(&path.display().to_string()))
        .unwrap_or_default()
}

pub fn process_matches(name: &str, path: &str, process: &Process) -> bool {
    if has_usable_path(path) {
        let exe = process_exe_path(process);
        return !exe.is_empty() && exe == normalize_windows_path(path);
    }

    if is_generic_name(name) {
        return false;
    }

    same_process_name(name, process.name())
}

#[derive(Debug, Clone)]
pub struct LiveProcessMatch {
    pub instance_count: u32,
    pub memory: u64,
}

pub fn find_running_matches(name: &str, path: &str) -> LiveProcessMatch {
    let mut sys = System::new();
    sys.refresh_processes();

    let mut instance_count = 0u32;
    let mut memory = 0u64;
    for process in sys.processes().values() {
        if process_matches(name, path, process) {
            instance_count += 1;
            memory = memory.saturating_add(process.memory());
        }
    }

    LiveProcessMatch {
        instance_count,
        memory,
    }
}

pub fn end_matching_processes(name: &str, path: &str) -> Result<u32, String> {
    if is_protected_process(name) {
        return Err("系统关键或安全相关进程不能结束".to_string());
    }

    let mut sys = System::new();
    sys.refresh_processes();

    let current_pid = sysinfo::get_current_pid().ok();
    let mut targets = Vec::new();
    for process in sys.processes().values() {
        if !process_matches(name, path, process) {
            continue;
        }
        if is_protected_process(process.name()) {
            return Err("匹配到系统关键或安全相关进程，已取消结束".to_string());
        }
        if current_pid == Some(process.pid()) {
            return Err("不能结束 CleanPilot 自己".to_string());
        }
        let exe = process_exe_path(process);
        if exe.contains(r"\windows\system32\") || exe.contains(r"\windows\syswow64\") {
            return Err("匹配到系统目录中的进程，已取消结束".to_string());
        }
        targets.push(process.pid());
    }

    if targets.is_empty() {
        return Ok(0);
    }

    let mut ended = 0;
    for pid in targets {
        if let Some(process) = sys.process(pid) {
            if process.kill() {
                ended += 1;
            }
        }
    }

    if ended == 0 {
        return Err("结束进程失败".to_string());
    }

    Ok(ended)
}
