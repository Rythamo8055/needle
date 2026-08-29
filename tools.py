import os
import subprocess
import needle


def _run(cmd):
    """Run a command and return stripped output."""
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True, timeout=10)
        return result.stdout.strip() or result.stderr.strip()
    except Exception as e:
        return f"Error: {e}"


@needle.tool
def battery_status():
    """Get battery status, percentage, and health."""
    out = _run("cat /sys/class/power_supply/BAT*/status 2>/dev/null || echo 'No battery found'")
    pct = _run("cat /sys/class/power_supply/BAT*/capacity 2>/dev/null || echo 'N/A'")
    health = _run("cat /sys/class/power_supply/BAT*/health 2>/dev/null || echo 'N/A'")
    return f"Status: {out} | Level: {pct}% | Health: {health}"


@needle.tool
def trash_size():
    """Get the size of files in trash."""
    trash_dir = os.path.expanduser("~/.local/share/Trash")
    if not os.path.exists(trash_dir):
        return "Trash directory not found"
    size = _run(f"du -sh {trash_dir} 2>/dev/null | cut -f1")
    files = _run(f"ls {trash_dir}/files 2>/dev/null | wc -l")
    return f"Trash size: {size} | Files: {files}"


@needle.tool
def cpu_temperature():
    """Get CPU temperature."""
    temp = _run("sensors 2>/dev/null | grep -i 'core 0' | awk '{print $3}'")
    if not temp or "Error" in temp:
        temp = _run("cat /sys/class/thermal/thermal_zone*/temp 2>/dev/null | head -1")
        if temp and temp.isdigit():
            temp = f"{int(temp) / 1000:.1f}°C"
    return f"CPU Temperature: {temp}" if temp else "Temperature sensor not found"


@needle.tool
def cpu_info():
    """Get CPU model, cores, and load average."""
    model = _run("grep 'model name' /proc/cpuinfo | head -1 | cut -d: -f2 | xargs")
    cores = _run("nproc")
    load = _run("cat /proc/loadavg | awk '{print $1, $2, $3}'")
    return f"CPU: {model} | Cores: {cores} | Load: {load}"


@needle.tool
def memory_usage():
    """Get RAM and swap usage."""
    mem = _run("free -h | grep Mem")
    swap = _run("free -h | grep Swap")
    return f"RAM: {mem}\nSwap: {swap}"


@needle.tool
def disk_usage():
    """Get disk usage for all mounted filesystems."""
    out = _run("df -h --type=ext4 --type=btrfs --type=xfs --type=vfat 2>/dev/null | grep -v tmpfs")
    return out


@needle.tool
def disk_health():
    """Get disk health via S.M.A.R.T. status."""
    out = _run("sudo smartctl -H /dev/sda 2>/dev/null | grep -i 'overall'")
    if not out or "Error" in out:
        out = _run("smartctl -H /dev/nvme0n1 2>/dev/null | grep -i 'overall'")
    return f"Disk Health: {out}" if out else "SMART not available (try running with sudo)"


@needle.tool
def network_info():
    """Get network interfaces and traffic."""
    out = _run("ip -br addr show | grep -v lo")
    traffic = _run("cat /proc/net/dev | grep -v lo | grep -v face | awk '{print $1\": RX=\"int($2/1048576)\"MB TX=\"int($10/1048576)\"MB\"}'")
    return f"Interfaces:\n{out}\n\nTraffic:\n{traffic}"


@needle.tool
def top_processes():
    """Get top 5 processes by CPU and memory usage."""
    cpu = _run("ps aux --sort=-%cpu | head -6 | tail -5 | awk '{printf \"%-6s %-5s%% CPU  %s\\n\", $1, $3, $11}'")
    mem = _run("ps aux --sort=-%mem | head -6 | tail -5 | awk '{printf \"%-6s %-5s%% MEM  %s\\n\", $1, $4, $11}'")
    return f"Top by CPU:\n{cpu}\n\nTop by Memory:\n{mem}"


@needle.tool
def process_count():
    """Get the number of running processes."""
    count = _run("ls /proc | grep -c '^[0-9]'")
    return f"Running processes: {count}"


@needle.tool
def uptime_info():
    """Get system uptime and boot time."""
    up = _run("uptime -p")
    boot = _run("who -b | awk '{print $3, $4}'")
    return f"Uptime: {up}\nBoot: {boot}"


@needle.tool
def hostname_info():
    """Get system hostname and kernel version."""
    host = _run("hostname")
    kernel = _run("uname -r")
    distro = _run("cat /etc/os-release | grep PRETTY_NAME | cut -d'\"' -f2")
    return f"Host: {host} | Kernel: {kernel}\nDistro: {distro}"


@needle.tool
def gpu_info():
    """Get GPU information."""
    out = _run("lspci | grep -i vga")
    if not out:
        out = _run("lspci | grep -i '3d controller'")
    return f"GPU: {out}" if out else "No GPU detected"


@needle.tool
def brightness():
    """Get and set screen brightness."""
    current = _run("cat /sys/class/backlight/*/brightness 2>/dev/null || echo 'N/A'")
    max_val = _run("cat /sys/class/backlight/*/max_brightness 2>/dev/null || echo 'N/A'")
    return f"Brightness: {current}/{max_val}"


@needle.tool
def list_files(path: str = "."):
    """List files in a directory."""
    if path in ("current_directory", "current dir", "here"):
        path = "."
    entries = os.listdir(path)
    return "\n".join(entries[:30])  # limit to 30


@needle.tool
def system_info():
    """Get full system information (uname -a)."""
    return _run("uname -a")


ALL_TOOLS = [
    battery_status, trash_size, cpu_temperature, cpu_info,
    memory_usage, disk_usage, disk_health, network_info,
    top_processes, process_count, uptime_info, hostname_info,
    gpu_info, brightness, list_files, system_info
]
