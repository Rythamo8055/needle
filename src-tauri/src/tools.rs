use std::process::Command;
use std::sync::Mutex;

static SUDO_PASSWORD: Mutex<Option<String>> = Mutex::new(None);
static SUDO_CACHED: Mutex<bool> = Mutex::new(false);

fn run(cmd: &str) -> String {
    let output = Command::new("sh").arg("-c").arg(cmd).output();
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&o.stderr).trim().to_string();
            if !stdout.is_empty() {
                stdout
            } else {
                stderr
            }
        }
        Err(e) => format!("Error: {}", e),
    }
}

pub fn check_sudo() -> bool {
    let output = Command::new("sudo").args(["-n", "true"]).output();
    let cached = output.map(|o| o.status.success()).unwrap_or(false);
    *SUDO_CACHED.lock().unwrap() = cached;
    cached
}

pub fn set_sudo_password(password: &str) -> bool {
    use std::io::Write;
    use std::process::Stdio;
    let mut child = match Command::new("sudo")
        .args(["-S", "true"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return false,
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all((password.to_string() + "\n").as_bytes());
    }
    let output = child.wait_with_output();
    let success = output.map(|o| o.status.success()).unwrap_or(false);
    if success {
        *SUDO_CACHED.lock().unwrap() = true;
        *SUDO_PASSWORD.lock().unwrap() = Some(password.to_string());
        let _ = Command::new("sudo").args(["-v"]).output();
    }
    success
}

fn run_sudo(cmd: &str) -> String {
    let cached = *SUDO_CACHED.lock().unwrap();
    if !cached {
        return "Sudo not authenticated".to_string();
    }
    let password = SUDO_PASSWORD.lock().unwrap().clone();
    let full_cmd = if password.is_some() {
        // Password is cached, but sudo timestamp should be valid after set_sudo_password.
        // Just run with sudo; timestamp is fresh.
        format!("sudo {}", cmd.strip_prefix("sudo ").unwrap_or(cmd))
    } else {
        // No password stored, rely on cached timestamp
        format!("sudo {}", cmd.strip_prefix("sudo ").unwrap_or(cmd))
    };
    run(&full_cmd)
}

// ── Tools ──

pub fn battery_status() -> String {
    let status = run("cat /sys/class/power_supply/BAT*/status 2>/dev/null | head -1 || echo 'No battery'");
    let pct = run("cat /sys/class/power_supply/BAT*/capacity 2>/dev/null | head -1 || echo 'N/A'");
    let health = run("cat /sys/class/power_supply/BAT*/health 2>/dev/null | head -1 || echo 'N/A'");
    format!("Status: {} | Level: {}% | Health: {}", status, pct, health)
}

pub fn trash_size() -> String {
    let trash = format!("{}/.local/share/Trash", std::env::var("HOME").unwrap_or_default());
    if !std::path::Path::new(&trash).exists() {
        return "Trash directory not found".to_string();
    }
    let size = run(&format!("du -sh {} 2>/dev/null | cut -f1", trash));
    let files = run(&format!("ls {}/files 2>/dev/null | wc -l", trash));
    format!("Trash size: {} | Files: {}", size.trim(), files.trim())
}

pub fn cpu_temperature() -> String {
    let temp = run("sensors 2>/dev/null | grep -i 'core 0' | awk '{print $3}'");
    if !temp.is_empty() && !temp.contains("Error") {
        return format!("CPU Temperature: {}", temp);
    }
    let raw = run("cat /sys/class/thermal/thermal_zone0/temp 2>/dev/null | head -1");
    if let Ok(val) = raw.trim().parse::<i32>() {
        return format!("CPU Temperature: {:.1}°C", val as f32 / 1000.0);
    }
    "Temperature sensor not found".to_string()
}

pub fn cpu_info() -> String {
    let model = run("grep 'model name' /proc/cpuinfo | head -1 | cut -d: -f2 | xargs");
    let cores = run("nproc");
    let load = run("cat /proc/loadavg | awk '{print $1, $2, $3}'");
    format!("CPU: {} | Cores: {} | Load: {}", model, cores.trim(), load)
}

pub fn memory_usage() -> String {
    let mem = run("free -h | grep Mem");
    let swap = run("free -h | grep Swap");
    format!("RAM: {}\nSwap: {}", mem, swap)
}

pub fn disk_usage() -> String {
    run("df -h --type=ext4 --type=btrfs --type=xfs --type=vfat 2>/dev/null | grep -v tmpfs")
}

pub fn disk_health() -> String {
    let cached = *SUDO_CACHED.lock().unwrap();
    if !cached {
        return "NEED_SUDO".to_string();
    }
    let out = run_sudo("smartctl -H /dev/sda 2>/dev/null | grep -i 'overall'");
    if !out.is_empty() && !out.contains("Error") && !out.contains("Sudo") {
        return format!("Disk Health: {}", out);
    }
    let out2 = run_sudo("smartctl -H /dev/nvme0n1 2>/dev/null | grep -i 'overall'");
    if !out2.is_empty() && !out2.contains("Error") && !out2.contains("Sudo") {
        return format!("Disk Health: {}", out2);
    }
    if out2.contains("NEED_SUDO") || out.contains("NEED_SUDO") {
        return "NEED_SUDO".to_string();
    }
    "SMART not available".to_string()
}

pub fn network_info() -> String {
    let addrs = run("ip -br addr show 2>/dev/null | grep -v lo");
    let traffic = run("cat /proc/net/dev | grep -v lo | grep -v face | awk '{print $1\": RX=\"int($2/1048576)\"MB TX=\"int($10/1048576)\"MB\"}'");
    format!("Interfaces:\n{}\n\nTraffic:\n{}", addrs, traffic)
}

pub fn top_processes() -> String {
    let cpu = run("ps aux --sort=-%cpu | head -6 | tail -5 | awk '{printf \"%-6s %-5s%% CPU  %s\\n\", $1, $3, $11}'");
    let mem = run("ps aux --sort=-%mem | head -6 | tail -5 | awk '{printf \"%-6s %-5s%% MEM  %s\\n\", $1, $4, $11}'");
    format!("Top by CPU:\n{}\n\nTop by Memory:\n{}", cpu, mem)
}

pub fn process_count() -> String {
    let count = run("ls /proc | grep -c '^[0-9]'");
    format!("Running processes: {}", count.trim())
}

pub fn uptime_info() -> String {
    let up = run("uptime -p");
    let boot = run("who -b 2>/dev/null | awk '{print $3, $4}'");
    format!("Uptime: {}\nBoot: {}", up, boot)
}

pub fn hostname_info() -> String {
    let host = run("hostname");
    let kernel = run("uname -r");
    let distro = run("cat /etc/os-release 2>/dev/null | grep PRETTY_NAME | cut -d'\"' -f2");
    format!("Host: {} | Kernel: {}\nDistro: {}", host.trim(), kernel.trim(), distro.trim())
}

pub fn gpu_info() -> String {
    let out = run("lspci 2>/dev/null | grep -i vga");
    if !out.is_empty() {
        return format!("GPU: {}", out);
    }
    let out2 = run("lspci 2>/dev/null | grep -i '3d controller'");
    if !out2.is_empty() {
        return format!("GPU: {}", out2);
    }
    "No GPU detected".to_string()
}

pub fn brightness() -> String {
    let cur = run("cat /sys/class/backlight/*/brightness 2>/dev/null | head -1 || echo 'N/A'");
    let max = run("cat /sys/class/backlight/*/max_brightness 2>/dev/null | head -1 || echo 'N/A'");
    format!("Brightness: {}/{}", cur.trim(), max.trim())
}

pub fn list_files(path: &str) -> String {
    let p = match path {
        "current_directory" | "current dir" | "here" => ".",
        _ => path,
    };
    let entries = std::fs::read_dir(p);
    match entries {
        Ok(dir) => {
            let mut out = String::new();
            for (i, entry) in dir.enumerate() {
                if i >= 30 { break; }
                if let Ok(e) = entry {
                    if let Some(name) = e.file_name().to_str() {
                        out.push_str(name);
                        out.push('\n');
                    }
                }
            }
            if out.is_empty() { "Empty directory".to_string() } else { out.trim().to_string() }
        }
        Err(e) => format!("Error: {}", e),
    }
}

pub fn system_info() -> String {
    run("uname -a")
}

pub fn get_time() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let secs = now.as_secs();
    // Use date command for formatted time
    run(&format!("date -d @{} '+%Y-%m-%d %H:%M:%S'", secs))
}
