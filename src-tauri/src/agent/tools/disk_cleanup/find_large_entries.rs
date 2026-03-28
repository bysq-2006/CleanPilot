use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use walkdir::WalkDir;

use crate::agent::runtime::AgentRuntime;
use crate::agent::tools::{ToolDefinition, ToolFuture};

#[derive(Deserialize)]
struct FindLargeEntriesArgs {
    path: String,
    min_size_mb: u64,
}

pub fn register() -> ToolDefinition {
    ToolDefinition {
        name: "find_large_entries",
        description: "递归扫描指定目录，列出大于指定 MB 阈值的文件和文件夹。",
        usage: "arguments 传 JSON 字符串，例如 {\"path\":\"C:/Users\",\"min_size_mb\":100}",
        handler: call,
    }
}

fn call(_runtime: AgentRuntime, payload: String) -> ToolFuture {
    Box::pin(async move {
        let args: FindLargeEntriesArgs =
            serde_json::from_str(&payload).map_err(|e| format!("参数解析失败: {}", e))?;
        let path_str = args.path.trim();
        let min_size_bytes = args.min_size_mb.saturating_mul(1024 * 1024);

        if path_str.is_empty() {
            return Err("目录路径不能为空".to_string());
        }

        let root = Path::new(path_str);
        if !root.exists() {
            return Err(format!("目录不存在: {}", root.display()));
        }

        if !root.is_dir() {
            return Err(format!("目标不是目录: {}", root.display()));
        }

        let mut directory_sizes: HashMap<PathBuf, u64> = HashMap::new();
        let mut large_files: Vec<(u64, String)> = Vec::new();
        let mut skipped_paths = Vec::new();

        for entry in WalkDir::new(root) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    skipped_paths.push(format!("- 遍历跳过: {}", e));
                    continue;
                }
            };
            let entry_path = entry.path();
            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(e) => {
                    skipped_paths.push(format!(
                        "- 元数据跳过: {} | 路径: {}",
                        e,
                        entry_path.display()
                    ));
                    continue;
                }
            };

            if metadata.is_file() {
                let file_size = metadata.len();

                if file_size >= min_size_bytes {
                    large_files.push((
                        file_size,
                        format!(
                            "- 类型: 文件 | 大小: {} 字节 | 路径: {}",
                            file_size,
                            entry_path.display()
                        ),
                    ));
                }

                let mut current = entry_path.parent();
                while let Some(parent) = current {
                    if !parent.starts_with(root) {
                        break;
                    }

                    *directory_sizes.entry(parent.to_path_buf()).or_insert(0) += file_size;
                    current = parent.parent();
                }
            } else if metadata.is_dir() {
                directory_sizes.entry(entry_path.to_path_buf()).or_insert(0);
            }
        }

        let mut large_directories = directory_sizes
            .into_iter()
            .filter(|(_, size)| *size >= min_size_bytes)
            .map(|(path, size)| {
                (
                    size,
                    format!(
                        "- 类型: 文件夹 | 大小: {} 字节 | 路径: {}",
                        size,
                        path.display()
                    ),
                )
            })
            .collect::<Vec<(u64, String)>>();

        large_files.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
        large_directories.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));

        let mut lines = vec![format!(
            "扫描目录: {}\n筛选条件: 大于等于 {} MB",
            root.display(),
            args.min_size_mb
        )];

        if large_directories.is_empty() && large_files.is_empty() {
            lines.push("未找到满足条件的文件或文件夹。".to_string());
            return Ok(lines.join("\n"));
        }

        if !large_directories.is_empty() {
            lines.push("文件夹结果：".to_string());
            lines.extend(large_directories.into_iter().map(|(_, line)| line));
        }

        if !large_files.is_empty() {
            lines.push("文件结果：".to_string());
            lines.extend(large_files.into_iter().map(|(_, line)| line));
        }

        if !skipped_paths.is_empty() {
            lines.push(format!(
                "注意：有 {} 个路径因权限不足或读取失败被跳过，以下为部分示例：",
                skipped_paths.len()
            ));
            lines.extend(skipped_paths.into_iter().take(20));
        }

        Ok(lines.join("\n"))
    })
}
