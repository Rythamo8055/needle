use crate::tools;
use std::process::Command;

pub struct ToolResult {
    pub tool: String,
    pub output: String,
}

// Store last model confidence/reasoning for lib.rs
static mut LAST_CONFIDENCE: f32 = 0.0;
static mut LAST_REASONING: Option<String> = None;

fn call_model(query: &str) -> Option<(Vec<(String, serde_json::Value)>, f32, String)> {
    // Try to find binary
    let candidates = [
        "src-tauri/binaries/needle-x86_64-unknown-linux-gnu",
        "src-tauri/binaries/needle",
        "binaries/needle-x86_64-unknown-linux-gnu",
        "/home/rythamo/from rahul laptop/development/just do it for fun/needle/src-tauri/binaries/needle-x86_64-unknown-linux-gnu",
    ];
    let bin = candidates.iter().find(|p| std::path::Path::new(p).exists())?;
    let tools_path = if std::path::Path::new("src-tauri/tools.json").exists() {
        "src-tauri/tools.json"
    } else if std::path::Path::new("tools.json").exists() {
        "tools.json"
    } else {
        "/home/rythamo/from rahul laptop/development/just do it for fun/needle/src-tauri/tools.json"
    };

    let output = Command::new(bin)
        .args(["--tools", tools_path, "--tool-index", "src-tauri/tools.idx", "--prompt", query])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Try parse JSON
    let v: serde_json::Value = serde_json::from_str(&stdout).ok()?;
    let confidence = v.get("confidence").and_then(|c| c.as_f64()).unwrap_or(0.0) as f32;
    let reasoning = v.get("reasoning").and_then(|r| r.as_str()).unwrap_or("").to_string();
    let success = v.get("success").and_then(|s| s.as_bool()).unwrap_or(false);

    // If truncated or failed, fallback to keyword
    if !success && v.get("error_code").and_then(|e| e.as_str()) == Some("truncated") {
        // Try single tool: take first from reasoning
        // For now, fallback
        return None;
    }

    let calls = v.get("function_calls")?.as_array()?;
    let mut result = Vec::new();
    for call in calls {
        let name = call.get("name")?.as_str()?.to_string();
        let args = call.get("arguments").cloned().unwrap_or(serde_json::Value::Object(Default::default()));
        result.push((name, args));
    }
    if result.is_empty() {
        return None;
    }
    unsafe {
        LAST_CONFIDENCE = confidence;
        LAST_REASONING = Some(reasoning.clone());
    }
    Some((result, confidence, reasoning))
}

fn match_tools(query: &str) -> Vec<&'static str> {
    let q = query.to_lowercase();
    let mut scores: Vec<(&str, i32)> = Vec::new();
    let candidates: Vec<(&str, Vec<&str>)> = vec![
        ("battery_status", vec!["battery", "charge", "health", "power"]),
        ("trash_size", vec!["trash", "bin", "deleted", "recycle"]),
        ("cpu_temperature", vec!["temperature", "temp", "thermal", "heat"]),
        ("cpu_info", vec!["cpu", "processor", "load average", "cores"]),
        ("memory_usage", vec!["memory", "ram", "swap"]),
        ("disk_usage", vec!["disk usage", "storage", "space", "free space", "disk space"]),
        ("disk_health", vec!["disk health", "smart", "health", "disk check"]),
        ("network_info", vec!["network", "interface", "traffic", "ip", "wifi"]),
        ("top_processes", vec!["top", "process", "cpu usage", "memory usage top"]),
        ("process_count", vec!["how many processes", "process count", "running processes"]),
        ("uptime_info", vec!["uptime", "boot", "how long", "running since"]),
        ("hostname_info", vec!["hostname", "host", "kernel", "distro", "os"]),
        ("gpu_info", vec!["gpu", "graphics", "vga"]),
        ("brightness", vec!["brightness", "screen", "backlight"]),
        ("list_files", vec!["list files", "directory", "ls", "files in"]),
        ("system_info", vec!["system", "uname"]),
        ("get_time", vec!["time", "date", "clock", "what time"]),
    ];
    for (name, keywords) in candidates {
        let mut score = 0;
        for kw in keywords {
            if q.contains(kw) {
                score += if kw.len() > 6 { 3 } else { 2 };
                if kw.contains(' ') && q.contains(kw) { score += 2; }
            }
        }
        if score > 0 { scores.push((name, score)); }
    }
    scores.sort_by(|a, b| b.1.cmp(&a.1));
    if scores.is_empty() { return vec![]; }
    let multi = q.contains(" and ") || q.contains(',');
    let take = if multi { 3 } else { 1 };
    scores.into_iter().take(take).map(|(n, _)| n).collect()
}

pub fn route(query: &str) -> Vec<ToolResult> {
    // Try real model first
    if let Some((calls, _conf, _reason)) = call_model(query) {
        let mut results = Vec::new();
        for (name, args) in calls {
            let output = match name.as_str() {
                "battery_status" => tools::battery_status(),
                "trash_size" => tools::trash_size(),
                "cpu_temperature" => tools::cpu_temperature(),
                "cpu_info" => tools::cpu_info(),
                "memory_usage" => tools::memory_usage(),
                "disk_usage" => tools::disk_usage(),
                "disk_health" => tools::disk_health(),
                "network_info" => tools::network_info(),
                "top_processes" => tools::top_processes(),
                "process_count" => tools::process_count(),
                "uptime_info" => tools::uptime_info(),
                "hostname_info" => tools::hostname_info(),
                "gpu_info" => tools::gpu_info(),
                "brightness" => tools::brightness(),
                "list_files" => {
                    let path = args.get("path").and_then(|p| p.as_str()).unwrap_or(".");
                    let path = match path {
                        "current_directory" | "current dir" | "here" => ".",
                        _ => path,
                    };
                    tools::list_files(path)
                }
                "system_info" => tools::system_info(),
                "get_time" => tools::get_time(),
                _ => continue,
            };
            results.push(ToolResult { tool: name, output });
        }
        if !results.is_empty() { return results; }
    }

    // Fallback to keyword matching
    let tool_names = match_tools(query);
    if tool_names.is_empty() { return vec![]; }
    let mut results = Vec::new();
    for name in tool_names {
        let output = match name {
            "battery_status" => tools::battery_status(),
            "trash_size" => tools::trash_size(),
            "cpu_temperature" => tools::cpu_temperature(),
            "cpu_info" => tools::cpu_info(),
            "memory_usage" => tools::memory_usage(),
            "disk_usage" => tools::disk_usage(),
            "disk_health" => tools::disk_health(),
            "network_info" => tools::network_info(),
            "top_processes" => tools::top_processes(),
            "process_count" => tools::process_count(),
            "uptime_info" => tools::uptime_info(),
            "hostname_info" => tools::hostname_info(),
            "gpu_info" => tools::gpu_info(),
            "brightness" => tools::brightness(),
            "list_files" => {
                let path = extract_path(query);
                tools::list_files(&path)
            }
            "system_info" => tools::system_info(),
            "get_time" => tools::get_time(),
            _ => continue,
        };
        results.push(ToolResult { tool: name.to_string(), output });
    }
    // Set fallback confidence
    unsafe {
        LAST_CONFIDENCE = 0.75;
        LAST_REASONING = Some(format!("Fallback keyword match for '{}'", query));
    }
    results
}

fn extract_path(query: &str) -> String {
    for word in query.split_whitespace() {
        if word.starts_with('/') || word.starts_with('.') {
            return word.trim_matches(|c| c == '"' || c == '\'' || c == ',' || c == '.').to_string();
        }
    }
    ".".to_string()
}

pub fn confidence(_query: &str, results: &[ToolResult]) -> f32 {
    if results.is_empty() { return 0.0; }
    unsafe { LAST_CONFIDENCE }
}

pub fn reasoning() -> String {
    unsafe { LAST_REASONING.clone().unwrap_or_default() }
}
