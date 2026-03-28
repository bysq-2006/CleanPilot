use serde::Deserialize;

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::{ToolDefinition, ToolFuture};

#[derive(Deserialize)]
struct HttpRequestArgs {
    method: String,
    url: String,
    body: Option<String>,
}

pub fn register() -> ToolDefinition {
    ToolDefinition {
        name: "http_request",
        description: "使用本机网络发起一个基础 HTTP 请求，适合作为联网能力的保底工具。支持 GET、POST、PUT、DELETE、PATCH。",
        usage: "arguments 传 JSON 字符串，例如 {\"method\":\"GET\",\"url\":\"https://www.baidu.com/s?wd=Rust\"}。国内常用搜索站点示例：百度 https://www.baidu.com/s?wd=关键词，必应中国 https://cn.bing.com/search?q=关键词，搜狗 https://www.sogou.com/web?query=关键词，360 搜索 https://www.so.com/s?q=关键词。",
        handler: call,
    }
}

fn call(_runtime: AgentRuntime, payload: String) -> ToolFuture {
    Box::pin(async move {
        let args: HttpRequestArgs =
            serde_json::from_str(&payload)
                .map_err(|e| format!("状态码: N/A\n参数解析失败: {}", e))?;
        let method = args.method.trim().to_ascii_uppercase();
        let url = args.url.trim().to_string();

        if method.is_empty() {
            return Err("状态码: N/A\nHTTP method 不能为空".to_string());
        }

        if url.is_empty() {
            return Err("状态码: N/A\nURL 不能为空".to_string());
        }

        let client = reqwest::Client::new();
        let mut request = match method.as_str() {
            "GET" => client.get(&url),
            "POST" => client.post(&url),
            "PUT" => client.put(&url),
            "DELETE" => client.delete(&url),
            "PATCH" => client.patch(&url),
            _ => return Err(format!("状态码: N/A\n不支持的 HTTP method: {}", method)),
        }
        .header("User-Agent", "CleanPilot-Agent/0.1 (+https://tauri.app)");

        if let Some(body) = args.body {
            request = request.body(body);
        }

        let response = request
            .send()
            .await
            .map_err(|e| {
                let status = e
                    .status()
                    .map(|code| code.to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                format!("状态码: {}\nHTTP 请求失败: {}", status, e)
            })?;

        let status = response.status();
        let final_url = response.url().to_string();
        let text = response
            .text()
            .await
            .map_err(|e| format!("状态码: {}\n读取响应正文失败: {}", status, e))?;

        if !status.is_success() {
            return Err(format!(
                "状态码: {}\n请求方法: {}\n最终 URL: {}\n响应正文:\n{}",
                status, method, final_url, text
            ));
        }

        Ok(format!(
            "请求方法: {}\n最终 URL: {}\n状态码: {}\n响应正文:\n{}",
            method, final_url, status, text
        ))
    })
}
