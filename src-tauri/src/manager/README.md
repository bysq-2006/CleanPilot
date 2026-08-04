# Manager 模块拓扑结构

## 目录与 `ManagerModule` 的关系

`ManagerModule` 定义在 `src-tauri/src/manager/mod.rs` 中，统一持有三个管理器。

```mermaid
flowchart TD
    A["src-tauri/src/manager/mod.rs"] --> B["ManagerModule"]

    B --> C["agent_scene: Arc<AgentSceneManager>"]
    B --> D["history: Arc<HistoryManager>"]
    B --> E["storage_box: Arc<StorageBoxManager>"]

    C --> C1["agent_scene.rs"]
    D --> D1["history.rs"]
    E --> E1["storage_box/mod.rs"]

    E1 --> F["storage_box/file_ops.rs"]
```

## 三个管理器的职责

```mermaid
flowchart LR
    M["ManagerModule"]

    M --> S["AgentSceneManager<br/>agent_scene.rs<br/><br/>主要功能：管理 Agent 场景、System Prompt 和可用工具"]
    M --> H["HistoryManager<br/>history.rs<br/><br/>主要功能：管理聊天会话和历史记录持久化"]
    M --> B["StorageBoxManager<br/>storage_box/mod.rs<br/><br/>主要功能：管理 Agent 生成的清理任务清单"]

    S --> S1["管理当前 Agent 场景"]
    S --> S2["切换 System Prompt"]
    S --> S3["配置可用工具"]

    H --> H1["管理会话 Context ID"]
    H --> H2["保存/加载聊天历史"]
    H --> H3["列出/删除历史记录"]

    B --> B1["保存清理任务清单"]
    B --> B2["读取/删除清单"]
    B --> B3["列出 Storage Box 记录"]
    B --> B4["调用 file_ops 执行文件操作"]
```

### `ManagerModule`

文件位置：`src-tauri/src/manager/mod.rs`

管理模块的总入口，负责创建、组合和协调三个管理器，同时监听 Agent 工具事件并触发清单保存。

### `AgentSceneManager`

文件位置：`src-tauri/src/manager/agent_scene.rs`

管理 Agent 当前业务场景，并根据场景更新 System Prompt 和可用工具。

### `HistoryManager`

文件位置：`src-tauri/src/manager/history.rs`

管理聊天会话上下文，以及历史记录的保存、恢复、列出和删除。

历史记录目录：`Tauri app_data_dir/history/`

### `StorageBoxManager`

文件位置：`src-tauri/src/manager/storage_box/mod.rs`

保存 Agent 生成的磁盘清理候选清单，并提供清单的读取、删除和列出功能。

Storage Box 目录：`Tauri app_data_dir/storage_box/`

### `file_ops`

文件位置：`src-tauri/src/manager/storage_box/file_ops.rs`

提供在 Windows 资源管理器中定位清单文件或目录等系统文件操作。

## 典型调用流程

```mermaid
flowchart TD
    A["Agent 分析磁盘"]
    B["write_storage_box_checklist 工具生成清单"]
    C["ManagerModule 接收事件"]
    D["StorageBoxManager 保存清单"]
    E["前端展示清单"]
    F["用户查看或确认清理项目"]
    G["file_ops 打开文件位置"]

    A --> B --> C --> D --> E --> F --> G
```
