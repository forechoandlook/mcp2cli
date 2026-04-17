# MCP2CLI (Rust Hub)

`mcp2cli` 是一个极致精简的命令行工具，用于管理和调用多个 **Model Context Protocol (MCP)** 服务器。它专为 **Code Agent** 优化，采用 CSV 输出格式以节省 Token。

## 🌟 核心特性

- **极致精简**：无异步运行时，无臃肿依赖，仅 150 行 Rust 核心逻辑。
- **Agent 优化**：
  - 全 CSV 格式输出。
  - `inspect --brief` 模式，剔除冗余描述，仅保留参数结构。
- **自动发现**：添加服务器时自动探测功能并生成描述。
- **内置缓存**：支持本地缓存，毫秒级响应 `list` 和 `inspect` 命令。
- **自更新**：内置 `update` 命令，自动同步 GitHub 最新版本。

## 🚀 快速安装

### macOS / Linux
```bash
curl -fsSL https://raw.githubusercontent.com/forechoandlook/mcp2cli/main/install.sh | bash
```

### Windows (PowerShell)
下载仓库中的 `mcp2cli-windows.exe` 即可使用。

---

## 🛠 使用手册

### 1. 全局管理
- **查看服务器列表 (CSV)**：
  ```bash
  mcp2cli list
  ```
- **添加服务器** (自动生成描述)：
  ```bash
  mcp2cli config add stitch <URL> <KEY>
  ```
- **自定义描述**：
  ```bash
  mcp2cli config set-desc stitch "我的 UI 设计专家"
  ```

### 2. 服务器操作 (以 stitch 为例)
- **列出工具**：
  ```bash
  mcp2cli stitch list
  ```
- **刷新缓存**：
  ```bash
  mcp2cli stitch list --refresh
  ```
- **简要查看工具参数**：
  ```bash
  mcp2cli stitch inspect create_project --brief
  ```
- **直接调用工具**：
  ```bash
  mcp2cli stitch create_project '{"title": "New Project"}'
  ```

### 3. 系统维护
- **查看版本**：
  ```bash
  mcp2cli version
  ```
- **一键更新**：
  ```bash
  mcp2cli update
  ```

## 🗑 卸载
```bash
curl -fsSL https://raw.githubusercontent.com/forechoandlook/mcp2cli/main/uninstall.sh | bash
```

---

## 📄 开源协议
MIT
