# Phone Doctor — Offline Diagnostics for $200 Android

**Target:** 3B users on 1GB RAM phones (India, Africa, SE Asia) — no data, no English.

**Stack:** Tauri mobile / Capacitor + Needle 2 (14MB, 28MB RAM) + Android `dumpsys` tools

**Tools (7, distinct for 90% reliability):**
- `battery_health` — wear, cycles, temp
- `storage_full` — `df`, large files, cache
- `memory_pressure` — `dumpsys meminfo`, kill background
- `network_slow` — signal, data usage
- `app_crash` — logcat
- `overheating` — thermal
- `time` — current

**Why offline:** Works on 2G, privacy (no data to cloud), fits on 1GB RAM where Qwen 0.6B cannot.

**Monetize:** Play Store $0.99 / OEM pre-install

**Next:** Port `src-tauri/src/tools.rs` Android variants (`termux-battery-status`, `dumpsys`) to `apps/phone-doctor/src-tauri/`
