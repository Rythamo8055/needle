use crate::tools;

pub struct ToolResult {
    pub tool: String,
    pub output: String,
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
                // exact phrase bonus
                if kw.contains(' ') && q.contains(kw) {
                    score += 2;
                }
            }
        }
        if score > 0 {
            scores.push((name, score));
        }
    }

    scores.sort_by(|a, b| b.1.cmp(&a.1));

    // Return top 3 if multiple matches, else top 1
    if scores.is_empty() {
        return vec![];
    }
    // If query has "and" assume multi-tool
    let multi = q.contains(" and ") || q.contains(',');
    let take = if multi { 3 } else { 1 };
    scores.into_iter().take(take).map(|(n, _)| n).collect()
}

pub fn route(query: &str) -> Vec<ToolResult> {
    let tool_names = match_tools(query);

    if tool_names.is_empty() {
        // Try to infer from query — fallback to get_time for time-like queries
        return vec![];
    }

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
                // Extract path from query if present
                let path = extract_path(query);
                tools::list_files(&path)
            }
            "system_info" => tools::system_info(),
            "get_time" => tools::get_time(),
            _ => continue,
        };
        results.push(ToolResult {
            tool: name.to_string(),
            output,
        });
    }
    results
}

fn extract_path(query: &str) -> String {
    // Very simple: look for "/" in query
    for word in query.split_whitespace() {
        if word.starts_with('/') || word.starts_with('.') {
            return word.trim_matches(|c| c == '"' || c == '\'' || c == ',' || c == '.').to_string();
        }
    }
    ".".to_string()
}

pub fn confidence(query: &str, results: &[ToolResult]) -> f32 {
    if results.is_empty() {
        return 0.0;
    }
    // Simple heuristic: if we matched, confidence is moderate
    // In real Needle, this comes from the model head
    let q = query.to_lowercase();
    let has_keyword = !match_tools(&q).is_empty();
    if has_keyword { 0.75 } else { 0.1 }
}
