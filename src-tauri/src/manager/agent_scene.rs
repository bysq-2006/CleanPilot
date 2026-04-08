use std::sync::{Arc, Mutex};

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::ToolManager;

const DEFAULT_AGENT_SCENE: AgentScene = AgentScene::DiskCleanup;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentScene {
    DiskCleanup,
}

impl AgentScene {
    pub fn from_str(value: &str) -> Result<Self, String> {
        match value {
            "disk_cleanup" => Ok(Self::DiskCleanup),
            _ => Err(format!("未知场景: {}", value)),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DiskCleanup => "disk_cleanup",
        }
    }

    pub fn scene_prompt(&self) -> &'static str {
        match self {
            Self::DiskCleanup => "当前场景为磁盘清理。请帮助用户分析磁盘空间占用，优先定位大目录、大文件和可清理项，并给出安全、明确的建议。开始时先查看磁盘总容量、已用空间和剩余空间；分析时遵循先轻扫、再深扫的流程，先快速浏览目录分布，再对重点目录做进一步扫描。遇到用途不明确的文件或目录时，可以联网检索其常见用途和清理风险。执行过程中，请简短说明你当前正在做什么。输出结果时尽量整理为清单，至少包含路径和可能用途，并补充建议、风险和预计可释放空间；最后说明当前剩余空间，以及清理候选项处理后理论上可达到的剩余空间。对于系统关键文件、重要个人数据或用途不明确的内容，应谨慎处理，优先提醒用户确认或备份。在收尾阶段，如果已经形成较稳定的清理候选清单，请调用 storage box 清单写入工具，把最终清单写入 storage box；写入内容中的每个元素只能包含 path 和 purpose。",
        }
    }

    pub fn enabled_tools(&self) -> &'static str {
        match self {
            Self::DiskCleanup => "list_directory,disk_info,find_large_entries,write_storage_box_checklist,file_read,http_request",
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
        let tool_prompt = tools.build_prompt();

        agent
            .history
            .system_prompt
            .lock()
            .map_err(|e| format!("Agent system prompt 锁获取失败: {}", e))?
            .set_tool_prompt(tool_prompt);

        let mut current_tools = agent
            .tools
            .lock()
            .map_err(|e| format!("Agent 工具锁获取失败: {}", e))?;
        *current_tools = tools;

        Ok(())
    }
}
