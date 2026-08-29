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

agent = needle.Needle(tools=[get_time])
response = agent.run(input("Ask: "))
print(response)
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
- [x] Set up local dev environment with `cactus-needle`
- [x] Run a basic tool-calling demo on laptop
- [x] Build a tools.json for our experiments
- [ ] Download binary for target platform
- [ ] Test on Android via Termux
- [ ] Explore custom tool integration

### Laptop Demo Results (2026-08-30)

All tools tested successfully. The model correctly selects the right tool based on the prompt.

| Prompt | Tool Called | Result |
|---|---|---|
| "what time is it" | `get_time` | `2026-08-30 00:07:24` |
| "how much disk space do I have" | `disk_usage` | `Total: 342GB \| Used: 38GB \| Free: 302GB` |
| "what is my memory usage" | `memory_usage` | `Total: 15662MB \| Used: 5759MB \| Available: 9903MB` |

**Performance:** ~160-240 decode TPS, peak RAM ~47-106 MB. Tool selection confidence varies (0.27-0.65) but accuracy is 100%.

### Decisions

1. **Start with the Python package on laptop** — easier to iterate and debug before moving to edge devices.
2. **Binary approach for mobile** — the Python package has dependencies that may not work on all phone architectures, so binary is the way to go for mobile.
3. **Tool-calling focus** — we won't use Needle 2 for chat/coding. It's purpose-built for tool selection from a defined set.
4. **Build a lightweight Linux system info tool** — the core idea: ask in plain English ("what is my battery health", "how much trash do I have"), get answers from hidden system info only accessible via terminal commands.

### What We've Done So Far

- [x] Researched Needle 2 capabilities and use cases
- [x] Created GitHub repo
- [x] Initialized devlog
- [x] Set up local dev environment with `cactus-needle`
- [x] Run a basic tool-calling demo on laptop
- [x] Build a tools.json for our experiments
- [x] Test retrieval with 10 tools — works correctly
- [x] Build the core Linux info tool with 16 system tools
- [x] Build GTK4 desktop app with search bar + quick buttons
- [x] Add in-app sudo password dialog for privileged tools
- [ ] Add multi-tool call support (chain multiple tools per query)
- [ ] Download binary for target platform
- [ ] Test on Android via Termux
- [ ] Explore custom tool integration

### Multi-Tool Calls

Needle 2 supports multi-step agentic loops (`max_steps=8` default). You can ask:
- "what time is it and how much disk space" → calls `get_time` + `disk_usage`
- "memory usage, hostname, and uptime" → calls 3 tools in sequence

The model chains tool calls automatically based on the prompt.

### Next Steps

- Build the core Linux info tool with 15-20 system tools
- Package it as a single command: `needle-ask "what is my battery health"`
- Add trash size, CPU temp, disk health, GPU info, etc.

### Stack (GTK — Scrapped 2026-08-30)

| Layer | Tech |
|---|---|
| UI | GTK4 (PyGObject) — scrapped |
| AI | Needle 2 (cactus-needle) |
| Backend | Python + subprocess |
| System Info | `/proc/*`, `lsblk`, `sensors`, `trash-cli` |
| Packaging | Single script or PyInstaller binary |

**Decision to scrap:** GTK was Linux-only and not multi-platform. Switched to Rust-based stack for true cross-platform support.

---

## Entry 1 — 2026-08-30 — Tauri 2 Migration

**Status:** In Progress — Rust Multi-Platform App

### Decision

Scrapped the Python/GTK app. New stack is **Tauri 2** — Rust backend + web frontend. Reason:
- True multi-platform: Linux, macOS, Windows, Android from one codebase
- Tiny bundle size, fast, secure (uses OS webview)
- Rust backend can directly call system commands (no Python dependency)
- Needle 2 binary can be embedded as sidecar

### New Stack (Final)

| Layer | Tech |
|---|---|
| UI | Tauri 2 + Vanilla JS (HTML/CSS) |
| Backend | Rust |
| AI | Needle 2 (keyword routing now, binary sidecar next) + Rust tool router |
| Tools | 17 Rust tools in `src-tauri/src/tools.rs` |
| System Info | `sh -c` via `std::process::Command` |
| Sudo | In-app modal, cached via `sudo -S` |
| Build | `npm run tauri dev` / `npm run tauri build` |

### What Changed

- Removed `app.py`, `needle_core.py`, `tools.py` (Python)
- Added `src-tauri/src/tools.rs` — 17 tools ported to Rust
- Added `src-tauri/src/needle.rs` — tool router (keyword matching, supports multi-tool `and`)
- Added `src-tauri/src/lib.rs` — Tauri commands: `ask`, `check_sudo`, `set_sudo`, `list_tools`
- New frontend `src/index.html` + `src/main.js` + `src/styles.css` with search, quick buttons, sudo modal

### How to Run (Tauri)

```bash
npm install
npm run tauri dev      # dev mode with hot reload
npm run tauri build    # production bundle
```

### Next Steps

- [ ] Replace keyword router with real Needle 2 binary sidecar
- [ ] Add 14MB needle binary to `src-tauri/binaries/`
- [ ] Test multi-tool: “battery and trash size”
- [ ] Package for Linux (.deb/.AppImage)
- [ ] Test on Android via `npm run tauri android dev`

---

*This devlog will be updated as the project progresses.*
