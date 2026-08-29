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

## Tauri App (Rust + Web)

```bash
npm install
npm run tauri dev     # run app
npm run tauri build   # build bundle
```

- 17 system tools, sudo dialog, multi-tool support
- Type in plain English: “battery health”, “trash size and cpu temp”

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
