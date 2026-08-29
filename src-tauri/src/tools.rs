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
            if !stdout.is_empty() { stdout } else { stderr }
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
    *SUDO_CACHED.lock().unwrap() = true;
    *SUDO_PASSWORD.lock().unwrap() = Some(password.to_string());
    let _ = Command::new("sudo").args(["-v"]).output();
    true
}

fn run_sudo(cmd: &str) -> String {
    let cached = *SUDO_CACHED.lock().unwrap();
    if !cached { return "Sudo not authenticated".to_string(); }
    let password = SUDO_PASSWORD.lock().unwrap().clone();
    let full_cmd = format!("sudo {}", cmd.strip_prefix("sudo ").unwrap_or(cmd));
    let _ = password;
    run(&full_cmd)
}

// ── 7 Tools ──

pub fn battery_status() -> String {
    let status = run("cat /sys/class/power_supply/BAT*/status 2>/dev/null | head -1 || echo 'No battery'");
    let pct = run("cat /sys/class/power_supply/BAT*/capacity 2>/dev/null | head -1 || echo 'N/A'");
    let health = run("cat /sys/class/power_supply/BAT*/health 2>/dev/null | head -1 || echo 'N/A'");
    let full = run("cat /sys/class/power_supply/BAT*/charge_full 2>/dev/null | head -1 || cat /sys/class/power_supply/BAT*/energy_full 2>/dev/null | head -1 || echo 0");
    let design = run("cat /sys/class/power_supply/BAT*/charge_full_design 2>/dev/null | head -1 || cat /sys/class/power_supply/BAT*/energy_full_design 2>/dev/null | head -1 || echo 0");
    let cycles = run("cat /sys/class/power_supply/BAT*/cycle_count 2>/dev/null | head -1 || echo N/A");
    let now = run("cat /sys/class/power_supply/BAT*/charge_now 2>/dev/null | head -1 || cat /sys/class/power_supply/BAT*/energy_now 2>/dev/null | head -1 || echo N/A");
    let manufacturer = run("cat /sys/class/power_supply/BAT*/manufacturer 2>/dev/null | head -1 || echo Unknown");
    let model = run("cat /sys/class/power_supply/BAT*/model_name 2>/dev/null | head -1 || echo Unknown");
    let wear = if let (Ok(f), Ok(d)) = (full.trim().parse::<f32>(), design.trim().parse::<f32>()) {
        if d > 0.0 { format!("{:.0}%", f / d * 100.0) } else { "N/A".to_string() }
    } else { "N/A".to_string() };
    format!(
        "Battery: {} {} | Level: {}% | Status: {} | Health: {} (Wear: {})\nDesign: {} | Current Max: {} | Now: {}\nCycles: {} | Reported Health: {}",
        manufacturer.trim(), model.trim(), pct.trim(), status.trim(), wear, wear, design.trim(), full.trim(), now.trim(), cycles.trim(), health.trim()
    )
}

pub fn disk_health() -> String {
    let dev = if std::path::Path::new("/dev/nvme0n1").exists() { "/dev/nvme0n1" } else { "/dev/sda" };
    let run_smart = |args: &str| -> String {
        let cached = *SUDO_CACHED.lock().unwrap();
        if cached {
            let out = run(&format!("sudo -n smartctl {} {} 2>/dev/null", args, dev));
            if !out.is_empty() && !out.contains("not authorized") { return out; }
            let out2 = run_sudo(&format!("smartctl {} {} 2>/dev/null", args, dev));
            if !out2.contains("Sudo not") && !out2.is_empty() { return out2; }
        }
        let out3 = run(&format!("sudo -n smartctl {} {} 2>/dev/null", args, dev));
        if !out3.is_empty() && !out3.contains("not authorized") { return out3; }
        let cmd_pkexec = format!("pkexec smartctl {} {} 2>/dev/null", args, dev);
        let out4 = run(&cmd_pkexec);
        if !out4.is_empty() && !out4.contains("not authorized") { return out4; }
        out4
    };
    let info = run_smart("-i");
    let health = run_smart("-H | grep -i 'overall\\|result'");
    let smart = run_smart("-A");
    if health.contains("not authorized") || health.contains("Sudo not") || health.is_empty() {
        return "Disk Health: Authentication required (pkexec dialog dismissed) or SMART not available".to_string();
    }
    let mut out = String::new();
    if !info.is_empty() {
        for line in info.lines() {
            let l = line.trim();
            if l.starts_with("Model Number") || l.starts_with("Serial Number") || l.starts_with("Firmware Version") || l.starts_with("Namespace 1 Size") {
                out.push_str(l); out.push('\n');
            }
        }
    }
    if !health.is_empty() { out.push_str(&format!("\nHealth: {}\n", health.trim())); }
    if !smart.is_empty() {
        for line in smart.lines() {
            let l = line.trim();
            if l.starts_with("Temperature:") || l.starts_with("Percentage Used") || l.starts_with("Power On Hours") || l.starts_with("Power Cycles") || l.starts_with("Available Spare") || l.starts_with("Data Units") || l.starts_with("Media and") {
                out.push_str(l); out.push('\n');
            }
        }
        if out.lines().count() < 5 {
            out.push_str("\n--- SMART Attributes ---\n");
            for line in smart.lines().take(20) { out.push_str(line); out.push('\n'); }
        }
    }
    if out.trim().is_empty() { "SMART not available".to_string() } else { out.trim().to_string() }
}

pub fn disk_usage() -> String {
    run("df -h --type=ext4 --type=btrfs --type=xfs --type=vfat 2>/dev/null | grep -v tmpfs")
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
