# Field Service Voice Forms — Offline Technician

**Target:** Field techs in basements/factories with no signal (elevator, HVAC, plumbing).

**Stack:** Tauri desktop + Android + Needle 2 extraction + `offline_sync_queue`

**Flow:** Voice "Otis elevator brake pad 3mm, replace next week" → `extract({"asset":"Otis-01","part":"brake_pad","measurement":"3mm","action":"replace"})` → `fill_form()` → queue → sync when online.

**Tools (7):**
- `extract_work_order` — structured extraction (Needle's Seal 32.6 strength)
- `fill_form`
- `take_photo`
- `scan_barcode`
- `offline_queue`
- `get_time`
- `list_files` (attachments)

**Why offline:** Basement = no signal, 20min typing saved, factory data never leaves device (air-gapped).

**Monetize:** $25/tech/month B2B SaaS

**Next:** Build `apps/field-service/src-tauri/src/tools.rs` with extraction schemas
