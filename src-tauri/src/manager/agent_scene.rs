use std::sync::{Arc, Mutex};

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::ToolManager;

const DEFAULT_AGENT_SCENE: AgentScene = AgentScene::DiskCleanup;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentScene {
    DiskCleanup,
    SpeedUp,
    General,
}

impl AgentScene {
    pub fn from_str(value: &str) -> Result<Self, String> {
        match value {
            "disk_cleanup" => Ok(Self::DiskCleanup),
            "speed_up" => Ok(Self::SpeedUp),
            "general" => Ok(Self::General),
            _ => Err(format!("未知场景: {}", value)),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DiskCleanup => "disk_cleanup",
            Self::SpeedUp => "speed_up",
            Self::General => "general",
        }
    }

    pub fn scene_prompt(&self) -> &'static str {
        match self {
            Self::DiskCleanup => "当前场景为磁盘清理。请帮助用户分析磁盘空间占用，优先定位大目录、大文件和可清理项，并给出安全、明确的建议。开始时先查看磁盘总容量、已用空间和剩余空间；仅在刚开始建立全局认知时，允许做一次简短的目录概览或轻量扫描，用来快速判断应该聚焦哪些位置。进入正式分析后，应尽量依赖那种递归遍历目录、按体积阈值筛出大文件和大文件夹的深度扫描工具，避免反复做浅层遍历或输出大批无效小文件。使用带筛选阈值的扫描能力时，应主动把阈值设得更高一些，只有大于该阈值的文件或文件夹才值得展示，尽量不要让 0B 文件、极小文件或无关结果占用上下文。不要依赖工具名字本身做僵硬判断。遇到用途不明确的文件或目录时，可以联网检索其常见用途和清理风险。执行过程中，请简短说明你当前正在做什么。输出结果时尽量整理为清单，至少包含路径和可能用途，并补充建议、风险和预计可释放空间；最后说明当前剩余空间，以及清理候选项处理后理论上可达到的剩余空间。对于系统关键文件、重要个人数据或用途不明确的内容，应谨慎处理，优先提醒用户确认或备份。在收尾阶段，如果已经形成较稳定的清理候选清单，请调用清单写入工具，把最终清单持久化保存；写入内容中的每个元素只能包含 path 和 purpose。写入 path 时应尽量保留扫描结果里的完整绝对路径，不要自行改写成简化路径、相对路径或只以 / 开头的路径；在 Windows 场景下，优先保留带盘符的完整路径，例如 C:/Users/Name/Downloads/a.zip。如果当前上下文里的候选路径不是完整绝对路径，或你无法确认盘符，就不要猜测，应先重新查看原始扫描结果，再决定是否写入。写入完成后，告知用户清理候选项已保存到「任务」，用户可以在任务页面查看并逐项确认是否删除，不要提及工具名称或任何内部组件名称。",
            Self::SpeedUp => "当前场景为「电脑变快」。请帮助用户诊断电脑卡顿、开机缓慢和资源占用问题，优先关注正在运行的高占用进程、开机启动项，也可以补充视觉效果、电源计划和磁盘占用这类常见原因。开始时先查看整体 CPU、内存、磁盘和电源计划；再根据情况列出占用较高的进程，以及开机启动项。当前还不能结束进程或修改启动项，因此不要假装已经改过系统设置。遇到用途不明确的程序时，可以联网检索其常见用途和关闭风险。请根据工具结果给出可操作的 Windows 优化建议，说明建议动作、风险和预期收益。对于系统关键进程、驱动、安全软件以及电源或热管理相关服务，应明确提醒不要随意结束或禁用。不要提及工具名称或任何内部组件名称。",
            Self::General => "当前场景为全能模式。你可以使用当前全部可用工具来帮助用户，不要把自己限定在单一任务上。如果用户想清理磁盘，就按磁盘清理的方式工作；如果用户想让电脑变快，就按卡顿诊断的方式工作；如果是其他问题，根据现有工具能力尽量完成。结论依赖外部信息、目录内容或文件状态时必须先调用工具，不能直接猜测。不要提及工具名称或任何内部组件名称。",
        }
    }

    pub fn enabled_tools(&self) -> &'static str {
        match self {
            Self::DiskCleanup => "list_directory,get_disk_info,find_large_entries,write_storage_box_checklist,file_read,http_request",
            Self::SpeedUp => "get_system_perf,list_processes,list_startup_items,file_read,http_request",
            Self::General => "*",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AgentSceneManager {
    current_scene: Arc<Mutex<AgentScene>>,
}

impl Default for AgentSceneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentSceneManager {
    pub fn new() -> Self {
        Self {
            current_scene: Arc::new(Mutex::new(DEFAULT_AGENT_SCENE)),
        }
    }

    /// 获取当前场景。
    pub fn get_current_scene(&self) -> Result<AgentScene, String> {
        self.current_scene
            .lock()
            .map(|scene| *scene)
            .map_err(|e| format!("Agent 场景锁获取失败: {}", e))
    }

    /// 切换到指定场景并应用到 Agent。
    pub fn switch_scene(&self, scene: AgentScene, agent: &AgentRuntime) -> Result<(), String> {
        let mut current_scene = self
            .current_scene
            .lock()
            .map_err(|e| format!("Agent 场景锁获取失败: {}", e))?;

        *current_scene = scene;

        agent
            .history
            .system_prompt
            .lock()
            .map_err(|e| format!("Agent system prompt 锁获取失败: {}", e))?
            .set_scene_prompt(scene.scene_prompt().to_string());

        let tools = ToolManager::new(scene.enabled_tools());

        let mut current_tools = agent
            .tools
            .lock()
            .map_err(|e| format!("Agent 工具锁获取失败: {}", e))?;
        *current_tools = tools;

        Ok(())
    }
}
