# MCP2CLI 开发与指令上下文

`mcp2cli` 是一个基于 Rust 的极简命令行工具，旨在管理和交互多个 **Model Context Protocol (MCP)** 服务器。它特别针对 **Code Agent** 进行了优化，通过 CSV 输出格式最大化 Token 利用率。

## 🚀 项目概览

- **核心功能**：MCP 服务器的配置（别名管理）、工具发现（list/inspect）和动态调用（call）。
- **架构设计**：采用“动态服务器分发”模式，允许直接使用别名作为子命令（例如 `mcp2cli stitch list`）。

## 🛠 构建与运行

### 发布流程
该项目采用 **Git Tag 驱动** 的发布自动化：
1. 更新代码并推送到主分支。
2. 创建版本标签：`git tag vX.Y.Z`。
3. 推送标签：`git push origin vX.Y.Z`。
4. **GitHub Actions** 会自动构建各平台二进制文件（Linux, macOS, Windows）并创建 Release。包括x86和arm平台.

## 📂 目录结构与关键文件

- `src/main.rs`: 核心逻辑，包含所有命令的处理、JSON-RPC 请求构造和 CSV 格式化输出。
- `build.rs`: 编译时脚本，动态从 Git 标签或环境变量注入版本号。
- `.github/workflows/release.yml`: 自动化构建与发布流水线。
- `install.sh` / `uninstall.sh`: 提供给用户的自动化安装和卸载脚本。

## 🤖 Agent 交互协议

在与此项目交互时，应遵循以下 **“Agent 优先”** 原则：

1. **发现路径**：
    - 先用 `list` 发现服务器。
    - 再用 `<alias> list` 发现工具。
    - 必要时用 `inspect <tool> --brief` 学习最小参数集。
2. **输出解析**：
    - 始终假设 `stdout` 为纯净的 CSV 格式。
    - 状态信息（如缓存提示、网络进度）会定向到 `stderr`。
3. **性能优化**：
    - 默认利用本地缓存。
    - 只有在明确需要更新工具定义时才建议用户使用 `--refresh`。

## 📜 开发规范

- **极简主义**：保持代码量精简（目前核心逻辑约 150 行），避免引入重型依赖（如 Tokio/Reqwest）。
- **同步 I/O**：除非有极高并发需求，否则坚持使用同步阻塞 I/O 以保持二进制体积小巧和逻辑确定性。
- **自描述**：利用 `summarize` 辅助函数自动从服务器返回的工具集中提取摘要，减少人工维护文档的需求。
