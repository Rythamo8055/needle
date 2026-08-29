# Field Service — Devlog

> Voice forms for offline technicians — structured extraction

## Entry 0 — 2026-08-30 — Planned

**Status:** Not started

**Idea:** Tech in basement with no signal says "Otis brake pad 3mm, replace next week" → extract `{"asset":"Otis-01","part":"brake_pad"}` → fill form → offline queue → sync.

**Stack:** Tauri desktop+Android + Needle 2 Seal extraction (32.6 vs 270M 16.3) + offline_sync_queue

**Tools (7 planned):** extract_work_order, fill_form, take_photo, scan_barcode, offline_queue, get_time, list_files

**Master:** See `/DEVLOG.md` for Needle core.
