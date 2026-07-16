# Diary — Project Analysis

Generated: 2026-07-15

---

## What It Is

**Diary** is a minimal desktop app for daily journaling + habit tracking. Offline-first, single-user, cozy book-like aesthetic. No cloud, no auth, no sync.

---

## Project Stage

**Phase 1 — near completion.** The scaffold is fully in place. All core features are built:

| Feature | Status |
|---|---|
| CodeMirror 6 editor with debounced autosave | Done |
| Habit definitions + per-day check tracking | Done |
| View switch pill (Today / Monthly) | Done |
| Custom title bar (macOS + Windows traffic lights) | Done |
| Dark mode via `prefers-color-scheme` | Done |
| Plugin-store persistence (journal + habits) | Done |
| Monthly view stub | Done (placeholder) |

**Git history**: 1 commit (`48b0460` — "stand up phase 1 foundation").

**Uncommitted changes** (3 files, minor polish):
- `ViewSwitch.svelte` — "Monthly" → "More", removed border
- `+layout.svelte` — border-radius 12px → 16px
- `+page.svelte` — date header font-size 13px → 16px

**Phase 2** (not started): Monthly calendar/analytics, vim motions, bullet lists.

---

## Tech Stack

| Layer | Choice |
|---|---|
| Desktop shell | **Tauri v2** (Rust backend) |
| Frontend framework | **SvelteKit 2** with **Svelte 5** runes mode |
| Adapter | `@sveltejs/adapter-static` (SPA, `ssr = false`) |
| Language | **TypeScript** (strict) |
| Package manager | **bun** |
| Styling | **Tailwind CSS v4** |
| Editor | **CodeMirror 6** (plain text only) |
| Font (journal) | **Newsreader** (serif) 17px / 1.7 |
| Font (UI) | **Atkinson Hyperlegible** (sans) 14px |
| Font delivery | `@fontsource/*` (bundled, no network) |
| State | Svelte 5 runes (`$state` / `$derived`) |
| Persistence | `@tauri-apps/plugin-store` (JSON file) |
| Dates | `date-fns` |
| Icons | SVG glyphs (inline), `lucide-svelte` available |

---

## Architecture

### Tauri app shell
- `src-tauri/src/main.rs` — entry point, calls `diary_lib::run()`
- `src-tauri/src/lib.rs` — registers plugins (opener, store), no custom commands yet
- Rust side is deliberately minimal — persistence is JS-only via plugin-store

### Frontend layout (SvelteKit SPA)
```
app.html → +layout.svelte → +page.svelte (Today)
                          → monthly/+page.svelte (Monthly stub)
```

### Component tree
```
+layout.svelte
├── TitleBar.svelte          — custom window controls (macOS/Win)
├── ViewSwitch.svelte        — Today / Monthly pill toggle
└── @render children()
    ├── +page.svelte         — Today view
    │   ├── Editor.svelte    — CodeMirror 6 wrapper
    │   └── HabitList.svelte — habit rail
    │       └── HabitRow.svelte — SVG checkbox + label
    └── monthly/+page.svelte — placeholder
```

### Store layer (3 files, one concern each)
| Store file | Owns |
|---|---|
| `journal.svelte.ts` | Date-keyed journal text `Record<YYYY-MM-DD, string>` |
| `habits.svelte.ts` | Habit definitions + per-day check states |
| `view.svelte.ts` | Active view (`'today'` / `'monthly'`) + navigation |

### Data flow
1. On mount, `+page.svelte` calls `initJournalStore()` + `initHabitsStore()`
2. Stores hydrate from `.diary-store.json` via Tauri plugin-store
3. Editor renders blank white until hydrated
4. User types → CodeMirror `updateListener` → debounce 400ms → `setEntry()`
5. Habit toggle → `toggleCheck()` → immediate persist
6. View switch → `setView()` → updates rune + navigates route

### Persistence schema (`.diary-store.json`)
```json
{
  "journal": { "2026-07-15": "Today I..." },
  "habits": {
    "habits": [
      { "id": "study", "label": "Study session", "createdAt": "..." },
      { "id": "gym", "label": "Go to gym", "createdAt": "..." },
      { "id": "read", "label": "Read 10 pages", "createdAt": "..." }
    ],
    "checks": {
      "study": { "2026-07-15": true }
    }
  }
}
```

### Window config
- Size: 1200×800 default, min 720×500
- `decorations: false` + `transparent: true` (custom title bar)
- Border-radius shell with Mac-style shadow
- CSS custom properties for light/dark via `prefers-color-scheme`

### Design system
- Single white (`#ffffff`) / dark (`#1e1e1e`) background
- Purple accent (`#8b7dbe` / `#a89bd9`)
- No gradients, no noise, no glossy UI
- 5 ink levels: `--ink`, `--ink-soft`, `--ink-faint`, `--line`, `--line-strong`

---

## File Tree (source only)

```
src/
├── app.css
├── app.html
├── lib/
│   ├── components/
│   │   ├── Editor.svelte
│   │   ├── HabitList.svelte
│   │   ├── HabitRow.svelte
│   │   ├── TitleBar.svelte
│   │   └── ViewSwitch.svelte
│   ├── editor/
│   │   ├── extensions.ts
│   │   └── theme.ts
│   └── stores/
│       ├── habits.svelte.ts
│       ├── journal.svelte.ts
│       └── view.svelte.ts
└── routes/
    ├── +layout.svelte
    ├── +layout.ts
    ├── +page.svelte
    └── monthly/
        └── +page.svelte
```

---

## Key Architectural Decisions

1. **No Rust commands in Phase 1** — persistence is JS-only via `@tauri-apps/plugin-store`
2. **One concern per store file** — journal, habits, and view are never cross-wired
3. **Editor created/destroyed in `onMount`** — never recreated on prop changes; content synced via `view.dispatch`
4. **Debounced writes (400ms)** — not per-keystroke, not unbounded
5. **Blank white until hydrated** — prevents flash-of-empty-editor
6. **Hydration on app launch** — `Promise.all` for both stores before rendering
7. **Fonts bundled locally** — works offline (no Google Fonts requests)
8. **`decorations: false`** — full custom chrome (needed for Mac-style traffic lights + shell)

---

## What's Missing / Next Steps

| Item | Priority |
|---|---|
| `bun run check` passes cleanly | Should verify |
| `cargo check` in src-tauri | Should verify |
| Phase 2: Monthly calendar/analytics | Future |
| Phase 2: Vim motions | Future |
| Phase 2: Bullet list auto-formatting | Future |
| Dark mode manual toggle | Future |
