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
    let v: serde_json::Value = serde_json::from_str(&stdout).ok()?;
    let confidence = v.get("confidence").and_then(|c| c.as_f64()).unwrap_or(0.0) as f32;
    let reasoning = v.get("reasoning").and_then(|r| r.as_str()).unwrap_or("").to_string();

    let calls = v.get("function_calls")?.as_array()?;
    let mut result = Vec::new();
    for call in calls {
        let name = call.get("name")?.as_str()?.to_string();
        let args = call.get("arguments").cloned().unwrap_or(serde_json::Value::Object(Default::default()));
        result.push((name, args));
    }
    unsafe {
        LAST_CONFIDENCE = confidence;
        LAST_REASONING = Some(reasoning.clone());
    }
    Some((result, confidence, reasoning))
}

pub fn route(query: &str) -> Vec<ToolResult> {
    // Pure model — no fallback
    let Some((calls, _conf, _reason)) = call_model(query) else {
        // Binary missing or parse failed — return empty, let lib.rs show error with 0 confidence
        unsafe {
            if LAST_REASONING.is_none() {
                LAST_CONFIDENCE = 0.0;
                LAST_REASONING = Some("Model not available or query failed".to_string());
            }
        }
        return vec![];
    };
    if calls.is_empty() {
        // Model returned no tool (e.g. "what time is it" with current tools) — return empty, real confidence
        return vec![];
    }
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
    results
}

pub fn confidence(_query: &str, results: &[ToolResult]) -> f32 {
    unsafe { LAST_CONFIDENCE }
}

pub fn reasoning() -> String {
    unsafe { LAST_REASONING.clone().unwrap_or_default() }
}
