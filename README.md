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

## Project Structure

```
needle/
├── DEVLOG.md          # Development log & decisions
├── tools.json         # Tool definitions for the binary
├── demo.py            # Laptop demo script
├── phone/
│   ├── tools.json     # Mobile tool definitions
│   └── demo.py        # Android/Termux script
└── README.md
```

## Devlog

See [DEVLOG.md](DEVLOG.md) for what we're building, why, and progress updates.

## License

MIT
