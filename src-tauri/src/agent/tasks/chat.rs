use crate::agent::history::AgentMessage;
use crate::agent::runtime::AgentRuntime;
use crate::agent::task_queue::AgentTask;

/// 处理用户问题任务，核心逻辑是：
/// 1. 将用户问题追加到历史记录中
/// 2. 基于当前历史记录调用 LLM 获取任务列表（不会对history 进行任何修改）
/// 3. 将 LLM 返回的任务列表解析成 AgentTask 并入队
pub async fn handle_user_question(runtime: &AgentRuntime, content: String) {
    if let Err(e) = runtime.history.append(AgentMessage {
        role: "user".to_string(),
        content: content.clone(),
    }) {
        eprintln!("Agent 写入历史记录失败: {}", e);
    }

    request_and_enqueue_tasks(
        runtime,
        "Agent 已根据用户问题生成任务列表",
        "Agent 任务入队失败",
    )
    .await;

    println!("Agent 收到用户问题任务: {}", content);
}

/// 当 agent 需要等待工具的回答才能继续思考时，使用这个任务
/// 这个任务的作用是：基于当前历史记录继续调用 LLM 获取新的任务列表，其他逻辑和 handle_user_question 基本一样
pub async fn handle_continue_reply(runtime: &AgentRuntime) {
    request_and_enqueue_tasks(
        runtime,
        "Agent 已根据历史记录生成后续任务列表",
        "Agent 继续回答任务入队失败",
    )
    .await;
}

async fn request_and_enqueue_tasks(
    runtime: &AgentRuntime,
    success_log: &str,
    enqueue_error_prefix: &str,
) {
    match runtime.llm.chat(&runtime.history).await {
        Ok(raw_reply) => match parse_llm_tasks(&raw_reply) {
            Ok(tasks) => {
                enqueue_tasks(runtime, tasks, enqueue_error_prefix);
                println!("{}: {}", success_log, raw_reply);
            }
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("LLM 调用失败: {}", e),
    }
}

fn enqueue_tasks(runtime: &AgentRuntime, tasks: Vec<AgentTask>, error_prefix: &str) {
    for task in tasks {
        if let Err(e) = runtime.tasks.push(task) {
            eprintln!("{}: {}", error_prefix, e);
            break;
        }
    }
}

pub fn parse_llm_tasks(raw: &str) -> Result<Vec<AgentTask>, String> {
    serde_json::from_str(raw).map_err(|e| {
        format!(
            "LLM 返回任务列表解析失败: {}\n原始输出: {}",
            e, raw
        )
    })
}

