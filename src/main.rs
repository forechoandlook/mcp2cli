use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{PathBuf};
use std::env;

const GITHUB_REPO: &str = "forechoandlook/mcp2cli";
const VERSION: &str = env!("APP_VERSION");

#[derive(Serialize, Deserialize, Default)]
struct Config {
    servers: HashMap<String, ServerConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ServerConfig {
    url: String,
    api_key: String,
    #[serde(default = "default_desc")]
    description: String,
}

fn default_desc() -> String { "No description".into() }

#[derive(Serialize)]
struct JsonRpcRequest { jsonrpc: String, id: u64, method: String, params: Value }

#[derive(Deserialize)]
struct JsonRpcResponse { result: Option<Value>, error: Option<Value> }

struct McpClient { url: String, api_key: String }

impl McpClient {
    fn new(url: String, api_key: String) -> Self { Self { url, api_key } }

    fn request(&self, method: &str, params: Value) -> Result<Value> {
        let resp: JsonRpcResponse = ureq::post(&self.url)
            .header("X-Goog-Api-Key", &self.api_key)
            .header("Content-Type", "application/json")
            .send_json(&JsonRpcRequest { jsonrpc: "2.0".into(), id: 1, method: method.into(), params })?
            .body_mut().read_json()?;

        if let Some(error) = resp.error { anyhow::bail!("MCP Error: {}", error); }
        Ok(resp.result.unwrap_or(Value::Null))
    }

    fn fetch_tools(&self) -> Result<Value> {
        let resp = self.request("tools/list", serde_json::json!({}))?;
        Ok(resp.get("tools").cloned().context("No tools in response")?)
    }
}

// --- Helpers ---

fn get_dirs() -> Result<ProjectDirs> { ProjectDirs::from("com", "mcp2cli", "mcp2cli").context("Dirs error") }

fn get_config_path() -> Result<PathBuf> { 
    let d = get_dirs()?; fs::create_dir_all(d.config_dir())?; Ok(d.config_dir().join("config.json")) 
}

fn get_cache_path(s: &str) -> Result<PathBuf> { 
    let d = get_dirs()?; fs::create_dir_all(d.cache_dir())?; Ok(d.cache_dir().join(format!("{}_tools.json", s))) 
}

fn get_tools_with_cache(alias: &str, client: &McpClient, force_refresh: bool) -> Result<Value> {
    let path = get_cache_path(alias)?;
    if !force_refresh && path.exists() {
        if let Ok(metadata) = fs::metadata(&path) {
            if let Ok(elapsed) = metadata.modified()?.elapsed() {
                let mins = elapsed.as_secs() / 60;
                let age = if mins == 0 { "Fresh".into() } else if mins < 60 { format!("{}m", mins) } else { format!("{}h", mins/60) };
                eprintln!("[cache] Age: {}", age);
            }
        }
        return Ok(serde_json::from_str(&fs::read_to_string(path)?)?);
    }
    eprintln!("[net] Fetching tools...");
    let tools = client.fetch_tools()?;
    fs::write(path, serde_json::to_string(&tools)?)?;
    Ok(tools)
}

fn summarize(tools: &Value) -> String {
    let arr = tools.as_array().map(|a| a.as_slice()).unwrap_or(&[]);
    if arr.is_empty() { return "No tools.".into(); }
    let names: Vec<&str> = arr.iter().take(5).filter_map(|t| t.get("name").and_then(|v| v.as_str())).collect();
    format!("Tools: {} {}", names.join("; "), if arr.len() > 5 { format!("(+{} more)", arr.len() - 5) } else { "".into() })
}

fn clean(s: &str) -> String { s.replace(',', ";").replace('\n', " ").replace('\r', "").trim().to_string() }

fn print_inspect(tool: &Value, brief: bool) {
    let name = tool.get("name").and_then(|v| v.as_str()).unwrap_or("?");
    let desc = tool.get("description").and_then(|v| v.as_str()).unwrap_or("");
    println!("METADATA\ntool_name,is_destructive,is_idempotent{}", if brief { "" } else { ",description" });
    println!("{},{},{},{}", name, false, false, if brief { "".into() } else { clean(desc) });

    println!("\nINPUT_PARAMETERS\nparameter,type,required{}", if brief { "" } else { ",description" });
    if let Some(props) = tool.get("inputSchema").and_then(|v| v.get("properties")).and_then(|v| v.as_object()) {
        let reqs = tool.get("inputSchema").and_then(|v| v.get("required")).and_then(|v| v.as_array());
        for (p, v) in props {
            let t = v.get("type").and_then(|v| v.as_str()).unwrap_or("any");
            let is_req = reqs.map_or(false, |r| r.contains(&serde_json::json!(p)));
            print!("{},{},{}", p, t, is_req);
            if !brief { print!(",{}", clean(v.get("description").and_then(|v| v.as_str()).unwrap_or(""))); }
            println!();
        }
    }
}

fn update() -> Result<()> {
    let api_url = format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO);
    let resp_val: Value = ureq::get(&api_url)
        .header("User-Agent", "mcp2cli-updater")
        .call()?
        .into_body()
        .read_json()?;
    
    let latest_v = resp_val.get("tag_name").and_then(|v| v.as_str()).context("Failed to get latest tag")?;
    let latest_v = latest_v.trim_start_matches('v');

    if latest_v == VERSION.trim_start_matches('v') {
        println!("Already up to date ({}).", VERSION);
        return Ok(());
    }

    println!("Updating from {} to {}...", VERSION, latest_v);
    
    let target = if cfg!(target_os = "macos") { "mcp2cli-macos" } 
                else if cfg!(target_os = "windows") { "mcp2cli-windows.exe" } 
                else { "mcp2cli-linux" };

    let bin_url = format!("https://github.com/{}/releases/download/{}/{}", GITHUB_REPO, latest_v, target);
    let mut resp = ureq::get(&bin_url).call()?;
    
    let mut bytes = Vec::new();
    resp.body_mut().as_reader().read_to_end(&mut bytes)?;

    let temp_path = env::temp_dir().join("mcp2cli_new");
    fs::write(&temp_path, &bytes)?;
    self_replace::self_replace(&temp_path)?;
    fs::remove_file(&temp_path).ok();

    println!("Successfully updated to {}.", latest_v);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("MCP CLI Hub (v{})\n\nGlobal:\n  list\n  update\n  version\n  config add <alias> <url> <key> [desc]\n  config set-desc <alias> <desc>\n  config remove <alias>\n\nServer:\n  <alias> list [--refresh]\n  <alias> inspect <tool> [--brief | --full]\n  <alias> <tool> [json_args]", VERSION);
        return Ok(());
    }

    let cfg_path = get_config_path()?;
    let mut cfg: Config = if cfg_path.exists() { serde_json::from_str(&fs::read_to_string(&cfg_path)?)? } else { Config::default() };

    match args[1].as_str() {
        "list" => {
            println!("alias,tools,description");
            let mut updated = false;
            for (name, srv) in &mut cfg.servers {
                let cache_path = get_cache_path(name)?;
                let tools = if cache_path.exists() { serde_json::from_str(&fs::read_to_string(cache_path)?)? } else { Value::Null };
                let count = tools.as_array().map(|a| a.len()).unwrap_or(0);
                if srv.description == "No description" && !tools.is_null() {
                    srv.description = summarize(&tools);
                    updated = true;
                }
                println!("{},{},{}", name, if count > 0 { count.to_string() } else { "---".into() }, clean(&srv.description));
            }
            if updated { fs::write(cfg_path, serde_json::to_string_pretty(&cfg)?)?; }
        }
        "update" => update()?,
        "version" => println!("mcp2cli version {}", VERSION),
        "config" => match args.get(2).map(|s| s.as_str()) {
            Some("add") if args.len() >= 6 => {
                let client = McpClient::new(args[4].clone(), args[5].clone());
                let desc = args.get(6).cloned().unwrap_or_else(|| summarize(&client.fetch_tools().unwrap_or(Value::Null)));
                cfg.servers.insert(args[3].clone(), ServerConfig { url: args[4].clone(), api_key: args[5].clone(), description: desc });
                fs::write(cfg_path, serde_json::to_string_pretty(&cfg)?)?;
            }
            Some("set-desc") if args.len() >= 5 => {
                if let Some(srv) = cfg.servers.get_mut(&args[3]) {
                    srv.description = args[4].clone();
                    fs::write(cfg_path, serde_json::to_string_pretty(&cfg)?)?;
                    println!("Description updated.");
                }
            }
            Some("remove") => { cfg.servers.remove(&args[3]); fs::write(cfg_path, serde_json::to_string_pretty(&cfg)?)?; }
            _ => println!("Invalid config command."),
        },
        alias => if let Some(srv) = cfg.servers.get(alias) {
            let client = McpClient::new(srv.url.clone(), srv.api_key.clone());
            match args.get(2).map(|s| s.as_str()).unwrap_or("list") {
                "list" => {
                    let tools = get_tools_with_cache(alias, &client, args.contains(&"--refresh".to_string()))?;
                    println!("tool_name,description");
                    for t in tools.as_array().unwrap() {
                        println!("{},{}", t["name"].as_str().unwrap(), clean(t["description"].as_str().unwrap_or("")));
                    }
                }
                "inspect" => {
                    let tool_name = args.get(3).context("Need tool name")?;
                    let tools = get_tools_with_cache(alias, &client, false)?;
                    let tool = tools.as_array().unwrap().iter().find(|t| t["name"] == tool_name.as_str()).context("Not found")?;
                    match args.get(4).map(|s| s.as_str()) {
                        Some("--full") => println!("{}", serde_json::to_string_pretty(tool)?),
                        Some("--brief") => print_inspect(tool, true),
                        _ => print_inspect(tool, false),
                    }
                }
                tool_name => {
                    let json_args: Value = serde_json::from_str(args.get(3).map(|s| s.as_str()).unwrap_or("{}"))?;
                    println!("{}", serde_json::to_string_pretty(&client.request("tools/call", serde_json::json!({"name": tool_name, "arguments": json_args}))?)?);
                }
            }
        } else { println!("Unknown server."); }
    }
    Ok(())
}
