mod needle;
mod needle_ffi;
mod tools;

use serde::Serialize;

#[derive(Serialize)]
struct AskResponse {
    results: Vec<String>,
    tools: Vec<String>,
    confidence: f32,
    reasoning: String,
}

#[tauri::command]
fn ask(query: String) -> AskResponse {
    let results = needle::route(&query);
    let confidence = needle::confidence(&query, &results);
    let model_reasoning = needle::reasoning();

    if results.is_empty() {
        return AskResponse {
            results: vec!["No matching tool found. Try: battery, trash, cpu, memory, disk, network, etc.".to_string()],
            tools: vec![],
            confidence,
            reasoning: if model_reasoning.is_empty() { "No tool matched the query.".to_string() } else { model_reasoning },
        };
    }

    // Check for sudo need
    for r in &results {
        if r.output == "NEED_SUDO" || r.output.contains("Authentication required") {
            return AskResponse {
                results: vec![r.output.clone()],
                tools: vec![r.tool.clone()],
                confidence,
                reasoning: if model_reasoning.is_empty() { format!("Tool {} requires sudo.", r.tool) } else { model_reasoning },
            };
        }
    }

    let tools_used: Vec<String> = results.iter().map(|r| r.tool.clone()).collect();
    let outputs: Vec<String> = results.iter().map(|r| r.output.clone()).collect();
    let reasoning = if model_reasoning.is_empty() { format!("Matched {} tool(s) for query.", tools_used.join(", ")) } else { model_reasoning };

    AskResponse {
        results: outputs,
        tools: tools_used,
        confidence,
        reasoning,
    }
}

#[tauri::command]
fn check_sudo() -> bool {
    tools::check_sudo()
}

#[tauri::command]
fn set_sudo(password: String) -> bool {
    tools::set_sudo_password(&password)
}

#[tauri::command]
fn list_tools() -> Vec<String> {
    vec![
        "battery_status".to_string(),
        "disk_health".to_string(),
        "disk_usage".to_string(),
        "uptime_info".to_string(),
        "hostname_info".to_string(),
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![ask, check_sudo, set_sudo, list_tools])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
