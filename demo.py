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
def list_files(path: str):
    """List files in a directory."""
    entries = os.listdir(path)
    return "\n".join(entries)


agent = needle.Agent(tools=[get_time, disk_usage, memory_usage, list_files])
response = agent.run(input("Ask: "))
print(response.result)
