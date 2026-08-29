# Needle 2 — Devlog

> A tiny 45M parameter agentic LLM that runs locally on phones, Raspberry Pis, and edge devices.

---

## Entry 0 — 2026-08-30

**Status:** In Progress — Project Setup & Research Phase

### Context

Needle 2 is a tiny LLM with only **45M parameters** (14 MB binary, 28 MB RAM) optimized for **tool calling and structured extraction** on edge/mobile devices. No API calls, no internet needed. It runs as a single self-contained binary.

### Why Needle 2?

We watched a walkthrough (YouTube, 0:00–14:15) showing Needle 2 in action — running locally on an Android phone via Termux with no internet. The model decides which tool to call based on a prompt. Examples demonstrated:

- Battery status queries
- Vibration control
- Text-to-speech
- Volume adjustment

This sparked the idea: **can we explore and build on top of this tiny agentic model?**

### Key Technical Details

| Property | Value |
|---|---|
| Parameters | 45 million |
| Binary size | 14 MB |
| RAM usage | 28 MB |
| Primary use case | Tool calling & structured extraction |
| NOT for | Chatting, coding, long-form generation |
| Supported platforms | Linux x86_64, Android ARM64, Raspberry Pi, etc. |

### How to Use Needle 2

**Option 1 — Python Package (simplest)**
```bash
pip install cactus-needle
# or with uv
uv add cactus-needle
```

Then use it in a single Python file:
```python
import needle
from datetime import datetime as dt

@needle.tool
def get_time():
    """Get the current local date and time."""
    return dt.now().strftime("%Y-%m-%d %H:%M:%S")

agent = needle.Agent(tools=[get_time])
response = agent.run(input("Ask: "))
print(response.result)
```

**Option 2 — Binary directly (for edge devices)**
1. Download the binary from [Hugging Face](https://huggingface.co) for your architecture
2. Create a `tools.json` defining available tools
3. Run:
```bash
./needle --tools tools.json --prompt "What time is it?"
```

### What We've Done So Far

- [x] Researched Needle 2 capabilities and use cases
- [x] Created GitHub repo
- [x] Initialized devlog
- [ ] Set up local dev environment with `cactus-needle`
- [ ] Run a basic tool-calling demo on laptop
- [ ] Download binary for target platform
- [ ] Build a tools.json for our experiments
- [ ] Test on Android via Termux
- [ ] Explore custom tool integration

### Decisions

1. **Start with the Python package on laptop** — easier to iterate and debug before moving to edge devices.
2. **Binary approach for mobile** — the Python package has dependencies that may not work on all phone architectures, so binary is the way to go for mobile.
3. **Tool-calling focus** — we won't use Needle 2 for chat/coding. It's purpose-built for tool selection from a defined set.

### Next Steps

- Install `cactus-needle` and run a minimal tool-calling example
- Define a `tools.json` with 3-4 simple tools (time, disk usage, memory, file listing)
- Write a wrapper script that routes tool calls to actual system commands

---

*This devlog will be updated as the project progresses.*
