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
        STATUS["status: AgentStatus<br/>记录 Idle 或 Chatting 状态"]
        EVENT["event_delegate: EventDelegate<br/>向 Manager 发送业务事件"]
    end

    RUNTIME --> HISTORY
    RUNTIME --> TASKS
    RUNTIME --> LLM
    RUNTIME --> TOOLS
    RUNTIME --> STATUS
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

    User->>Queue: push UserQuestion（查找 C 盘大文件）

    Runtime->>Queue: pop()
    Queue-->>Runtime: UserQuestion
    Runtime->>Handler: 分发 UserQuestion
    Handler->>History: 追加 user 消息
    Handler->>LLM: 使用 History 请求模型
    LLM-->>Handler: 返回 tool_calls(find_large_entries)
    Handler->>History: 保存 assistant 工具调用消息
    Handler->>Queue: push ToolCall
    Handler->>Queue: push ContinueFromToolResults

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
