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

取消机制的核心是 `AgentState`：它同时表示 Agent 的运行状态，并持有当前这轮对话的 `CancellationToken`。`Idle` 没有令牌，`Chatting { cancellation_token }` 则表示一轮请求正在执行，因此不会出现“显示正在运行但没有可取消令牌”之类的不一致状态。

用户发送问题时，`AgentRuntime` 调用 `AgentState::begin()` 创建本轮唯一的令牌，再把同一个令牌共享给 `UserQuestion` 以及随后产生的所有工具和继续回复任务。主动取消时，`AgentState::cancel()` 取消该令牌并回到 `Idle`；正常结束时，`finish_if_current()` 只允许持有当前令牌的任务结束本轮状态。

```mermaid
flowchart TD
    INPUT["用户发送问题"] --> BEGIN["AgentState::begin()<br/>创建本轮唯一 Token"]
    IDLE["AgentState::Idle<br/>无取消令牌"] --> BEGIN
    BEGIN --> CHATTING["AgentState::Chatting<br/>{ cancellation_token }"]

    CHATTING --> QUESTION["UserQuestion + 同一个 Token"]
    QUESTION --> TOOL["ToolCall + 同一个 Token"]
    TOOL --> CONTINUE["ContinueFromToolResults<br/>+ 同一个 Token"]

    CHATTING -->|用户取消| CANCEL["AgentState::cancel()<br/>取消 Token"]
    CANCEL --> STOP["当前任务感知取消<br/>队列与未完成上下文被清理"]
    STOP --> IDLE

    CONTINUE -->|本轮正常完成| FINISH["finish_if_current(Token)"]
    FINISH --> IDLE
```

各任务只负责传播并检查这枚共享令牌；`AgentRuntime` 负责跨组件收尾，包括清空待执行任务，以及保留已生成文本、移除未完成的工具调用上下文。
