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
            Self::DiskCleanup => "当前场景为磁盘清理。你的目标是帮助用户分析磁盘占用、定位大文件与目录，并给出清理建议。",
        }
    }

    pub fn enabled_tools(&self) -> &'static str {
        match self {
            Self::DiskCleanup => "list_directory,disk_info,find_large_entries,file_read,http_request",
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
