import needle
import os
import shutil
from datetime import datetime as dt


@needle.tool
def get_time():
    """Get the current local date and time."""
    return dt.now().strftime("%Y-%m-%d %H:%M:%S")


@needle.tool
def disk_usage():
    """Get disk space usage for the current directory."""
    total, used, free = shutil.disk_usage(".")
    return f"Total: {total // (1024**3)}GB | Used: {used // (1024**3)}GB | Free: {free // (1024**3)}GB"


@needle.tool
def memory_usage():
    """Get current RAM usage."""
    with open("/proc/meminfo") as f:
        lines = f.readlines()
    total = int(lines[0].split()[1]) // 1024
    available = int(lines[2].split()[1]) // 1024
    used = total - available
    return f"Total: {total}MB | Used: {used}MB | Available: {available}MB"


@needle.tool
def list_files(path: str = "."):
    """List files in a directory."""
    if path in ("current_directory", "current dir", "here"):
        path = "."
    entries = os.listdir(path)
    return "\n".join(entries)


@needle.tool
def cpu_info():
    """Get CPU information and load average."""
    with open("/proc/loadavg") as f:
        load = f.read().strip()
    with open("/proc/cpuinfo") as f:
        lines = f.readlines()
    model = [l for l in lines if "model name" in l]
    name = model[0].split(":")[1].strip() if model else "unknown"
    return f"CPU: {name} | Load: {load}"


@needle.tool
def network_info():
    """Get network interface information."""
    with open("/proc/net/dev") as f:
        lines = f.readlines()[2:]
    interfaces = []
    for line in lines:
        parts = line.split()
        iface = parts[0].rstrip(":")
        rx_bytes = int(parts[1])
        tx_bytes = int(parts[9])
        interfaces.append(f"{iface}: RX={rx_bytes // (1024**2)}MB TX={tx_bytes // (1024**2)}MB")
    return "\n".join(interfaces)


@needle.tool
def process_count():
    """Get the number of running processes."""
    count = len([d for d in os.listdir("/proc") if d.isdigit()])
    return f"Running processes: {count}"


@needle.tool
def uptime():
    """Get system uptime."""
    with open("/proc/uptime") as f:
        seconds = float(f.read().split()[0])
    days = int(seconds // 86400)
    hours = int((seconds % 86400) // 3600)
    mins = int((seconds % 3600) // 60)
    return f"Up {days}d {hours}h {mins}m"


@needle.tool
def current_user():
    """Get the current logged-in user."""
    import subprocess
    result = subprocess.run(["whoami"], capture_output=True, text=True)
    return result.stdout.strip()


@needle.tool
def hostname():
    """Get the system hostname."""
    import subprocess
    result = subprocess.run(["hostname"], capture_output=True, text=True)
    return result.stdout.strip()


agent = needle.Needle(
    tools=[get_time, disk_usage, memory_usage, list_files,
           cpu_info, network_info, process_count, uptime,
           current_user, hostname],
    tool_index_path="tools.idx"
)

response = agent.run(input("Ask: "))
print(response)
