# CleanPilot

> AI 驱动的 Windows 磁盘清理助手

Windows 自带的磁盘清理太保守，手动一个个文件夹翻又太费时间。CleanPilot 让 AI 替你做这件事——它能主动浏览你的磁盘、找出占用大的目录、读取文件内容辅助判断，最终整理出一份清单，你只需要决定删不删。

## 功能

- **AI 对话驱动**：用自然语言描述你想清理的目标，Agent 自动规划并执行分析
- **磁盘智能探索**：查看磁盘用量、列出目录内容、定位大文件/大文件夹
- **Storage Box 清单**：Agent 将可清理的路径整理成清单，保存供你随时回顾
- **一键在资源管理器中打开**：直接跳转到对应文件夹，手动确认后自行删除
- **可配置 LLM**：支持接入兼容 OpenAI 接口的模型

## 使用方法

1. 下载并双击安装包启动 CleanPilot
2. 打开右上角 **设置**，填入你的 API Key、Base URL 和模型名称
3. 回到主界面，用自然语言告诉 AI 你想清理什么，例如：
   - "帮我看看 C 盘哪里占用最大"
   - "清理一下 D 盘的下载文件夹"
4. AI 会自动探索磁盘并整理出可清理的路径清单（Storage Box）
5. 在 Storage Box 中查看清单，点击"打开文件夹"在资源管理器中确认后手动删除

## 获取 API Key

CleanPilot 支持任何兼容 OpenAI 接口的模型服务，以下是常见平台：

| 平台 | 地址 | 备注 |
|------|------|------|
| OpenAI | https://platform.openai.com/ | GPT 系列 |
| Anthropic | https://console.anthropic.com/ | Claude 系列 |
| DeepSeek | https://platform.deepseek.com/ | 国内可用，价格低 |
| 阿里云百炼 | https://bailian.console.aliyun.com/ | Qwen 系列，国内可用 |
| 月之暗面 | https://platform.moonshot.cn/ | Kimi 系列，国内可用 |
| 智谱 AI | https://open.bigmodel.cn/ | GLM 系列，国内可用 |

推荐国内用户优先考虑 DeepSeek 或阿里云百炼，价格实惠且无需代理。

## 数据存储

所有数据保存在本地，可在设置中查看存储目录的位置。目录结构如下：

```
存储目录/
├── config.json       # 应用设置（API Key、模型等）
├── history/          # 对话历史，每个会话一个 JSON 文件
└── storage_box/      # AI 整理的清理清单，每次生成一个 JSON 文件
```

## 技术栈

- [Tauri 2](https://tauri.app/) + Rust
- Vue 3 + TypeScript

## 路线图

- [ ] 联网能力：让 Agent 能够搜索网络，帮助判断某个文件/文件夹是否可以安全删除
- [ ] 更多系统管理功能，逐步演进为 AI 电脑管家
