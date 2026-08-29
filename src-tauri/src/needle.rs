use crate::needle_ffi;
use crate::tools;
use std::sync::Once;

pub struct ToolResult {
    pub tool: String,
    pub output: String,
}

static mut LAST_CONFIDENCE: f32 = 0.0;
static mut LAST_REASONING: Option<String> = None;
static INIT: Once = Once::new();

fn ensure_init() {
    INIT.call_once(|| {
        let tools_json = std::fs::read_to_string("src-tauri/tools.json")
            .or_else(|_| std::fs::read_to_string("tools.json"))
            .or_else(|_| std::fs::read_to_string("/home/rythamo/from rahul laptop/development/just do it for fun/needle/src-tauri/tools.json"))
            .unwrap_or("[]".to_string());
        let _ = needle_ffi::init(&tools_json, Some("src-tauri/tools.idx"));
    });
}

fn call_model(query: &str) -> Option<(Vec<(String, serde_json::Value)>, f32, String)> {
    ensure_init();
    needle_ffi::reset();
    let v = needle_ffi::complete(query, 256)?;
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
    for (name, _args) in calls {
        let output = match name.as_str() {
            "battery_status" => tools::battery_status(),
            "disk_health" => tools::disk_health(),
            "disk_usage" => tools::disk_usage(),
            "uptime_info" => tools::uptime_info(),
            "hostname_info" => tools::hostname_info(),
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
