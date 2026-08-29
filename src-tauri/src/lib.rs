mod needle;
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

    if results.is_empty() {
        return AskResponse {
            results: vec!["No matching tool found. Try: battery, trash, cpu, memory, disk, network, etc.".to_string()],
            tools: vec![],
            confidence,
            reasoning: "No tool matched the query.".to_string(),
        };
    }

    // Check for sudo need
    for r in &results {
        if r.output == "NEED_SUDO" {
            return AskResponse {
                results: vec!["NEED_SUDO".to_string()],
                tools: vec![r.tool.clone()],
                confidence,
                reasoning: format!("Tool {} requires sudo.", r.tool),
            };
        }
    }

    let tools_used: Vec<String> = results.iter().map(|r| r.tool.clone()).collect();
    let outputs: Vec<String> = results.iter().map(|r| r.output.clone()).collect();
    let reasoning = format!("Matched {} tool(s) for query.", tools_used.join(", "));

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
        "trash_size".to_string(),
        "cpu_temperature".to_string(),
        "cpu_info".to_string(),
        "memory_usage".to_string(),
        "disk_usage".to_string(),
        "disk_health".to_string(),
        "network_info".to_string(),
        "top_processes".to_string(),
        "process_count".to_string(),
        "uptime_info".to_string(),
        "hostname_info".to_string(),
        "gpu_info".to_string(),
        "brightness".to_string(),
        "list_files".to_string(),
        "system_info".to_string(),
        "get_time".to_string(),
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
