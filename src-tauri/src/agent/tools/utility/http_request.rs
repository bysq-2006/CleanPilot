use std::io::Cursor;
use std::io::Read;

use serde::Deserialize;
use ureq::Agent;

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::ToolDefinition;

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

fn call(_runtime: &AgentRuntime, payload: &str) -> Result<String, String> {
    let args: HttpRequestArgs =
        serde_json::from_str(payload).map_err(|e| format!("参数解析失败: {}", e))?;
    let method = args.method.trim().to_ascii_uppercase();
    let url = args.url.trim().to_string();

    if method.is_empty() {
        return Err("HTTP method 不能为空".to_string());
    }

    if url.is_empty() {
        return Err("URL 不能为空".to_string());
    }

    let client = Agent::new();

    let request = match method.as_str() {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        "PATCH" => client.patch(&url),
        _ => return Err(format!("不支持的 HTTP method: {}", method)),
    }
    .set("User-Agent", "CleanPilot-Agent/0.1 (+https://tauri.app)");

    let mut response = if let Some(body) = args.body {
        request
            .send(Cursor::new(body.into_bytes()))
            .map_err(|e| format!("HTTP 请求失败: {}", e))?
    } else {
        request.call().map_err(|e| format!("HTTP 请求失败: {}", e))?
    };

    let status = response.status();
    let final_url = response.get_url().to_string();
    let mut text = String::new();
    response
        .into_reader()
        .read_to_string(&mut text)
        .map_err(|e| format!("读取响应正文失败: {}", e))?;

    Ok(format!(
        "请求方法: {}\n最终 URL: {}\n状态码: {}\n响应正文:\n{}",
        method, final_url, status, text
    ))
}
