# Agent 模块结构与运行流程

## 1. AgentRuntime 持有的组件

```mermaid
flowchart LR
    RUNTIME["AgentRuntime<br/><br/>Agent 的运行时主体<br/>持有所有核心组件<br/>循环处理任务"]

    subgraph COMPONENTS[AgentRuntime 持有]
        direction TB

        HISTORY["history: AgentHistory<br/>保存系统提示词、对话和工具结果"]
        TASKS["tasks: AgentTaskQueue<br/>保存等待处理的 Agent 任务"]
        LLM["llm: AgentLlm<br/>读取配置并调用大模型"]
        TOOLS["tools: ToolManager<br/>管理并执行当前可用工具"]
        STATE["state: AgentState<br/>Idle 或 Chatting { cancellation_token }"]
        EVENT["event_delegate: EventDelegate<br/>向 Manager 发送业务事件"]
    end

    RUNTIME --> HISTORY
    RUNTIME --> TASKS
    RUNTIME --> LLM
    RUNTIME --> TOOLS
    RUNTIME --> STATE
    RUNTIME --> EVENT
```

## 2. 一次包含工具调用的运行流程

假设用户问：**“帮我找一下 C 盘占空间最大的文件。”**

```mermaid
sequenceDiagram
    actor User as 用户
    participant Queue as AgentTaskQueue<br/>核心任务队列
    participant Runtime as AgentRuntime<br/>循环取出任务
    participant Handler as tasks::handle_task<br/>按类型处理任务
    participant History as AgentHistory<br/>保存完整上下文
    participant LLM as AgentLlm<br/>调用大模型
    participant Tools as ToolManager<br/>执行工具

    User->>Runtime: 创建本轮 CancellationToken
    Runtime->>Queue: push UserQuestion（问题 + Token）

    Runtime->>Queue: pop()
    Queue-->>Runtime: UserQuestion
    Runtime->>Handler: 分发 UserQuestion
    Handler->>History: 追加 user 消息
    Handler->>LLM: 使用 History 请求模型
    LLM-->>Handler: 返回 tool_calls(find_large_entries)
    Handler->>History: 保存 assistant 工具调用消息
    Handler->>Queue: push ToolCall（携带同一个 Token）
    Handler->>Queue: push ContinueFromToolResults（携带同一个 Token）

    Note over Queue: 所有后续工作已经重新进入主队列

    Runtime->>Queue: pop()
    Queue-->>Runtime: ToolCall
    Runtime->>Handler: 分发 ToolCall
    Handler->>Tools: 执行 find_large_entries(C:/)
    Tools-->>Handler: 返回大文件和目录
    Handler->>History: 追加 tool 结果消息

    Runtime->>Queue: pop()
    Queue-->>Runtime: ContinueFromToolResults
    Runtime->>Handler: 分发继续回复任务
    Handler->>LLM: 使用包含工具结果的 History 再次请求
    LLM-->>Handler: 返回最终扫描说明和建议
    Handler->>History: 更新 assistant 最终消息

    User->>History: 前端查询历史
    History-->>User: 显示最终回复
```

`AgentRuntime` 不决定下一步做什么，它只负责反复执行同一件事：从 `AgentTaskQueue` 取出一个任务，再根据任务类型分发。模型产生工具调用后，也必须先重新进入队列，之后才会被运行时执行。

## 3. 请求取消机制

每次用户发送问题时，`AgentRuntime` 都会调用 `AgentState::begin()` 创建一个独立的 `CancellationToken`，并进入 `AgentState::Chatting { cancellation_token }`。这个令牌会跟随 `UserQuestion`、该轮产生的所有 `ToolCall` 以及 `ContinueFromToolResults` 一起进入任务队列。令牌只会从“可运行”变为“已取消”，不会被重置；下一轮请求会创建一个全新的令牌。

LLM 建连、流式回复循环、工具调用前后以及工具内部的目录遍历等长循环都会检查令牌。HTTP 请求和异步消息发送使用 `tokio::select!` 同时等待业务结果与取消通知，因此不必等到网络请求自然结束才响应取消。

```mermaid
sequenceDiagram
    actor User as 用户
    participant UI as ChatComposer
    participant Command as cancel_chat
    participant Runtime as AgentRuntime
    participant Token as CancellationToken
    participant Queue as AgentTaskQueue
    participant Worker as 当前 LLM / 工具任务
    participant History as AgentHistory

    User->>UI: 点击红色方形停止按钮
    UI->>Command: invoke("cancel_chat")
    Command->>Runtime: cancel_current()
    Runtime->>Token: cancel()
    Runtime->>Queue: clear()
    Runtime->>History: 保留已生成文本并清理未完成工具调用
    Runtime->>Runtime: status = Idle
    Token-->>Worker: cancelled
    Worker-->>Worker: 退出等待或终止循环
    UI->>Runtime: 轮询 get_agent_status
    Runtime-->>UI: idle
    UI-->>User: 恢复发送按钮
```

取消时会保留 assistant 已经生成的文本。若已经进入工具调用阶段，则清除未完成的 `tool_calls` 及其后续工具消息，避免下一轮请求携带缺少对应 `tool` 结果的无效消息组合。

前端通过共享的 `useAgentStatus` 状态轮询同时驱动工作提示和发送按钮：状态为 `chatting` 时显示红色方形终止按钮，状态为 `idle` 时显示普通发送按钮。

Runtime 使用一个状态值同时表达运行状态和当前令牌：`Idle` 不包含令牌，`Chatting` 必然包含令牌。这样避免了分别维护 `status` 与 `Option<CancellationToken>` 时可能出现的不一致组合。

`state.rs` 只负责 `AgentState` 自身的开始、取消、完成和状态查询；清空任务队列、整理取消后的历史记录等跨组件操作仍由 `AgentRuntime` 编排。
