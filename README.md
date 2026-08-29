# Needle

Experiments with [Needle 2](https://huggingface.co) — a tiny 45M parameter agentic LLM for tool calling on edge devices.

## What is Needle 2?

- **45M parameters** (14 MB binary, 28 MB RAM)
- Optimized for **tool calling** and **structured extraction**
- Runs locally — no API, no internet
- Supports Linux, Android (Termux), Raspberry Pi, and more

## Getting Started

### Python Package

```bash
pip install cactus-needle
```

### Binary

Download from [Hugging Face](https://huggingface.co) for your architecture:

```bash
chmod +x needle
./needle --help
```

## Tauri App (Rust + Web) — Real Needle 2 Inside

```bash
# 1. Download 15MB model binary (once)
python -c "from huggingface_hub import hf_hub_download; import shutil; p=hf_hub_download(repo_id='Cactus-Compute/needle2', filename='linux-x86_64/needle'); shutil.copyfile(p, 'src-tauri/binaries/needle-x86_64-unknown-linux-gnu')"
chmod +x src-tauri/binaries/needle-x86_64-unknown-linux-gnu

# 2. Run
npm install
npm run tauri dev     # real model: 0.84-0.97 confidence
npm run tauri build   # bundle
```

- 17 system tools, pkexec for disk health (fingerprint-safe), real 45M model
- Type in plain English: “battery health”, “trash size and cpu temp”
- Model picks tool, Rust executes: e.g. “disk health” → 0.97 → SMART full report

## Python Demos

```bash
pip install cactus-needle
python demo.py              # 4 tools
python demo_many_tools.py   # 16 tools, retrieval
```

## Project Structure

```
needle/
├── DEVLOG.md               # Development log & decisions
├── src/                    # Tauri frontend (HTML/CSS/JS)
├── src-tauri/              # Rust backend
│   ├── src/tools.rs        # 17 system tools
│   ├── src/needle.rs       # Tool router
│   └── src/lib.rs          # Tauri commands
├── tools.json              # Tool definitions
├── demo.py                 # Python demo (4 tools)
├── demo_many_tools.py      # Python demo (16 tools)
├── phone/                  # Android/Termux
└── README.md
```

## Devlog

See [DEVLOG.md](DEVLOG.md) for what we're building, why, and progress updates.

## License

MIT
