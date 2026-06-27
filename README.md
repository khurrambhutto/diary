# Diary

A minimal, cozy desktop app for daily journaling and habit tracking.
Single white background. Book-like. Not a productivity dashboard.

## Features

- **Today** (default view): write today's journal on the left 70%, tick off daily habits on the right 30%.
- **Monthly** (Phase 2, stub for now): calendar + analytics view.
- Offline-first — bundled fonts, local JSON store, no network required to write.
- Plain-text journal with the CodeMirror 6 editor (undo history, find, future-proofed for vim + bullets).

## Tech stack

Tauri v2 + SvelteKit 5 (runes) + TypeScript + Tailwind v4 + CodeMirror 6 + `@tauri-apps/plugin-store`.
Persistence is a local JSON file; no backend, no auth, no cloud sync.

## Getting started

```bash
bun install
bun run tauri dev
```

### Common commands

```bash
bun run dev          # Vite dev server (port 1420)
bun run tauri dev    # Full desktop dev shell
bun run build        # Production web build → ./build
bun run tauri build  # Build + bundle the desktop app
bun run check        # svelte-kit sync + svelte-check
```

## Project layout

- `src/routes/` — SvelteKit pages (`+page.svelte` = Today, `monthly/` = Monthly stub).
- `src/lib/components/` — `Editor`, `HabitList`, `ViewSwitch`.
- `src/lib/stores/` — `journal`, `habits`, `view` (one concern each).
- `src/lib/editor/` — CodeMirror extensions + theme (kept out of the component).
- `src-tauri/src/` — Rust entry (`main.rs` → `lib.rs`), feature modules added as needed.

See `AGENTS.md` for the full design system, conventions, and phase roadmap.

## License

MIT