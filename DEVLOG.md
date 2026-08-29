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

### Tools (17 total)

| # | Tool | Description | Sudo | Example Prompt |
|---|---|---|---|---|
| 1 | `battery_status` | Real wear (full/design), cycles, level, status | No | “battery health” |
| 2 | `trash_size` | Trash folder size + file count | No | “trash size” |
| 3 | `cpu_temperature` | CPU temp via sensors / thermal_zone | No | “cpu temperature” |
| 4 | `cpu_info` | Model, cores, load average | No | “cpu info” |
| 5 | `memory_usage` | RAM + swap (`free -h`) | No | “memory usage” |
| 6 | `disk_usage` | Mounted FS usage (`df -h`) | No | “disk usage” |
| 7 | `disk_health` | Full SMART: model, health, temp, % used, hours | Yes (pkexec) | “disk health” |
| 8 | `network_info` | Interfaces + RX/TX | No | “network info” |
| 9 | `top_processes` | Top 5 by CPU + memory | No | “top processes” |
| 10 | `process_count` | Total running processes | No | “how many processes” |
| 11 | `uptime_info` | Uptime + boot time | No | “uptime” |
| 12 | `hostname_info` | Host, kernel, distro | No | “hostname” |
| 13 | `gpu_info` | GPU via lspci | No | “gpu info” |
| 14 | `brightness` | Screen backlight level | No | “brightness” |
| 15 | `list_files` | List directory (30 entries) | No | “list files in /tmp” |
| 16 | `system_info` | `uname -a` | No | “system info” |
| 17 | `get_time` | Current date/time | No | “what time is it” |

### Sudo Handling

- Only `disk_health` needs sudo/pkexec (SMART requires root).
- Pre-fix: `sudo -S` hung on Fedora fingerprint → stuck modal.
- Fix: use `pkexec` (system polkit dialog handles fingerprint), fallback to `sudo -n` if cached. `set_sudo_password` now just caches, no blocking `sudo -S` call.

### Real Health Update (2026-08-30)

- `battery_status`: now shows wear % (3118/4211=74%), design/current/now mAh, cycles (252), manufacturer BYD.
- `disk_health`: now shows full SMART instead of just PASSED — model PM9B1 1TB, temp 41°C, 12% used, 7474h, 221 cycles, 59TB written.

### Next Steps

- [x] Show real health (wear, SMART details) — done
- [x] Replace keyword router with real Needle 2 binary sidecar — done (2026-08-30)
- [x] Download 15MB needle binary to `src-tauri/binaries/` — done
- [ ] Test multi-tool: “battery and trash size”
- [ ] Package for Linux (.deb/.AppImage)
- [ ] Test on Android via `npm run tauri android dev`

---

## Entry 2 — 2026-08-30 — Real Needle 2 Integration

**Status:** Done — Model now live in Tauri

### What Changed

- Downloaded `linux-x86_64/needle` (15MB, 14MB compressed) from `Cactus-Compute/needle2` to `src-tauri/binaries/needle-x86_64-unknown-linux-gnu`
- Created `src-tauri/tools.json` (17 tools) for binary
- Rewrote `src-tauri/src/needle.rs` to call binary via `Command::new(bin) --tools tools.json --tool-index tools.idx --prompt query`
- Parse `function_calls`, `confidence`, `reasoning` from JSON, execute matching Rust tool
- Fallback to keyword router if binary missing or returns empty (e.g. “what time is it” edge case)

### Real Model Results (binary, 17 tools)

| Prompt | Tool | Confidence | Reasoning |
|---|---|---|---|
| “what is my battery health” | `battery_status` | 0.84 | Query asks for battery health |
| “how much trash” | `trash_size` | 0.84 | Query asks for trash size |
| “cpu temperature” | `cpu_temperature` | 0.80 | Maps to get_cpu_temperature |
| “disk health” | `disk_health` | 0.97 | -> disk_health tool |
| “get the time” | `get_time` | 0.97 | Query asks for time info |
| “list files in /tmp” | `list_files` | 0.49 | `{"path":"/tmp"}` |

Fallback handles “what time is it” (keyword) when model returns empty.

### Download for New Clone

```bash
python -c "from huggingface_hub import hf_hub_download; import shutil; p=hf_hub_download(repo_id='Cactus-Compute/needle2', filename='linux-x86_64/needle'); shutil.copyfile(p, 'src-tauri/binaries/needle-x86_64-unknown-linux-gnu')"
chmod +x src-tauri/binaries/needle-x86_64-unknown-linux-gnu
```

---

*This devlog will be updated as the project progresses.*
