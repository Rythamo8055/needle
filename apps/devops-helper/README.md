# DevOps Helper — Plain-English System Admin (Pruned)

**This is the current root Tauri app, pruned to 7 distinct tools for 90% reliability.**

**From:** 17 tools (45% correct) → **To:** 7 tools (target 85-90%)

**Keep (distinct, high-value, not in Settings):**
- `battery_status` — wear 74%, cycles
- `disk_health` — SMART full (pkexec)
- `large_files` — find 10GB+ files (new, high-value)
- `port_kill` — what is using :3000 (new)
- `failed_services` — systemctl failed (new)
- `disk_usage` — df
- `get_time`

**Drop (overlap / low value):** `trash_size`, `brightness`, `gpu_info`, etc. (keep as quick-buttons only)

**Stack:** Current `src/` + `src-tauri/` (Tauri 2 + FFI `libneedle.so` 90ms, 27MB) — will be copied here on next iteration

**Next:** Prune `src-tauri/tools.json` 17→7 and re-stress test
