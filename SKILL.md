---
name: mcp2cli
description: "Manage and interact with Model Context Protocol (MCP) servers. Triggered by requests to list, inspect, or call MCP tools/servers."
metadata: 
  version: "0.1.0"
  authors: ["zwy"]
---
## Summary
- `mcp2cli` is a minimalist CLI for managing multiple MCP servers, optimized for Code Agents with CSV output.
- It supports server alias-based tool discovery and execution.

## 安装卸载
```bash
# Install (macOS/Linux)
curl -fsSL https://raw.githubusercontent.com/forechoandlook/mcp2cli/main/install.sh | bash
# Uninstall
curl -fsSL https://raw.githubusercontent.com/forechoandlook/mcp2cli/main/uninstall.sh | bash
```

## 基础用法
```bash
# List all configured servers
mcp2cli list
# Add a new MCP server
mcp2cli config add <alias> <url> <api_key> [description]
# List tools from a specific server
mcp2cli <alias> list [--refresh]
# Inspect a tool's parameters (CSV format)
mcp2cli <alias> inspect <tool_name> [--brief]
# Call a tool with JSON arguments
mcp2cli <alias> <tool_name> '{"param": "value"}'
```

## Workflow
1. **Discover Servers**: Use `mcp2cli list` to see available servers and their aliases.
2. **Discover Tools**: Use `mcp2cli <alias> list` to see tools available on a specific server.
3. **Understand Parameters**: Use `mcp2cli <alias> inspect <tool_name> --brief` to get a concise view of required parameters and types.
4. **Execute**: Call the tool using `mcp2cli <alias> <tool_name> '<json_args>'`. Ensure arguments are valid JSON.
5. **Update**: Use `mcp2cli update` to get the latest version from GitHub.

## 注意事项与约束
- **Output Format**: Most lists and inspection outputs are in CSV format for token efficiency.
- **Caching**: Tools are cached locally. Use `--refresh` with the `list` command if you suspect tools have changed on the server.
- **Error Handling**: Errors are directed to `stderr`. Always check for non-zero exit codes if running in scripts.
- **Security**: Never share your `api_key` or log it in plain text.
- **Brief Mode**: Prefer `inspect --brief` when discovering tool schemas to minimize token usage.
