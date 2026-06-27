# AGENTS.md — Diary

Instructions and conventions for AI coding agents (and humans) working on this project.
Read this before touching anything. Follow it unless you have a strong reason and explain deviations.

## What this app is

**Diary** is a minimal, cozy desktop app for **daily journaling + habit tracking**.
Single white background. Warm, calm, book-like — not a productivity dashboard.

Two pages, toggled by a pill-shaped switch at top-center of the window:

- **Today** (default, Phase 1) — the main view
  - Left 70% of the window: a writing surface for today's journal entry (plain text).
  - Right 30% of the window, centered: a vertical list of habits with checkbox labels
    (e.g. "Study session", "Go to gym", "Read 10 pages"). User ticks them off.
- **Monthly** (Phase 2 — skip charts for now) — calendar/analytics view.
  Build the shell and the toggle, leave the content area as a placeholder for now.

No multi-user, no cloud sync, no auth. Offline-first single-user desktop app.

## Tech stack (agreed)

| Concern            | Choice                                                            |
| ------------------ | ---------------------------------------------------------------- |
| Runtime / shell    | Tauri v2                                                          |
| Frontend framework | SvelteKit 2 with Svelte 5 (runes mode)                           |
| Adapter            | `@sveltejs/adapter-static` — SPA mode (`ssr = false`)            |
| Language           | TypeScript (strict)                                              |
| Package manager    | **bun** (`bun install`, `bun run`, `bun add`)                    |
| Styling            | Tailwind CSS v4                                                  |
| Editor             | CodeMirror 6 — plain text only in Phase 1                        |
| Font (journal body)| Newsreader (serif) 17px, line-height 1.7                         |
| Font (UI chrome)   | Atkinson Hyperlegible (sans) 14px, weights 400/700              |
| Fonts delivery     | `@fontsource/*` — bundled locally, no Google Fonts requests      |
| State              | Svelte 5 runes (`$state` / `$derived`) in component + stores     |
| Persistence        | `@tauri-apps/plugin-store` (JSON store)                          |
| Dates              | `date-fns`                                                       |
| Icons              | `lucide-svelte`                                                  |

### Why these choices

- **bun** because `bun.lock` already exists — do not switch to npm/yarn/pnpm.
- **Tailwind v4** for utility-first styling on a single-white-background design.
- **CodeMirror 6, not textarea** because future roadmap includes vim motions
  and bullet lists; CM6 lets us add those as single imports without re-platforming.
  Phase 1 still tree-shakes to ~40KB because we skip markdown/vim extensions.
- **@fontsource** so the app works offline (e.g. on a plane). A journaling app
  must never block writing on a network font fetch.
- **Svelte 5 runes** — the existing scaffold already uses `$state`, stay in runes mode.
  Do not use the legacy `let` reactivity pattern.

## Commands

```bash
bun run dev          # Vite dev server on port 1420 (strict)
bun run build        # Production build → ./build
bun run preview      # Preview the production build
bun run check        # svelte-kit sync + svelte-check (TS + a11y)
bun run check:watch  # Same, in watch mode
bun run tauri dev    # Full Tauri desktop dev shell (runs `bun run dev` first)
bun run tauri build  # Build + bundle the desktop app
```

### Before finishing any task

1. Run `bun run check` and fix every error/warning it reports.
2. If you changed anything in `src-tauri/`, run `cargo check` inside that dir.
3. Do not leave `console.log` / debug prints in committed code.

### Port

Vite is on **port 1420 strict** (Tauri expects this). Do not change it.

## Directory structure (target)

```
src/
  app.html
  app.css                          # Tailwind entry + base styles
  lib/
    components/
      Editor.svelte               # CodeMirror wrapper (journal surface)
      HabitList.svelte            # Right 30% — checkboxes
      ViewSwitch.svelte           # Top-center Today / Monthly pill
    stores/
      journal.svelte.ts            # date-keyed journal text (plugin-store)
      habits.svelte.ts            # habit definitions + check states
      view.svelte.ts              # 'today' | 'monthly'
    editor/
      extensions.ts                # CM6 extension preset (history, search, etc.)
      theme.ts                    # CM6 theme — Newsreader, line-wrap, padding
  routes/
    +layout.ts                    # `export const ssr = false`
    +layout.svelte                # Global font imports + layout shell
    +page.svelte                   # Today view (default)
    monthly/
      +page.svelte                # Monthly view (Phase 2 — stub only)
src-tauri/
  src/lib.rs                      # Tauri commands if any (start minimal)
  tauri.conf.json
```

Do not invent new top-level dirs without explanation. Keep the app flat and discoverable.

## Persistence model

One Tauri plugin-store file (e.g. `.diary-store.json`) lives in the app's data dir.

### Schema (Phase 1)

```ts
// journal entry text, one per day
type JournalStore = Record<string /* YYYY-MM-DD */, string /* entry body */>

// habit definitions + per-day check states
type HabitDef = { id: string; label: string; createdAt: string }
type HabitStore = {
  habits: HabitDef[]
  // outer key = habitId, inner key = YYYY-MM-DD, value = checked?
  checks: Record<string, Record<string, boolean>>
}
```

### Rules

- Read/write the store through thin stores in `src/lib/stores/*`. Do not call
  the plugin-store API directly from components.
- Journal autosave: debounce writes ~400ms after the last change in CodeMirror.
  Never write on every keystroke.
- On app launch, hydrate store state into the relevant `$state` rune before
  rendering the writing surface. Show nothing (blank white) until hydrated.
- Keys are always `YYYY-MM-DD` from `date-fns/format`. Never invent your own
  date string formats.

## Editor (CodeMirror) integration rules

This is the most error-prone part. Follow strictly:

1. **Create the `EditorView` in `onMount`, destroy it on cleanup.**
   Never create it in the component body.
2. **One `EditorView` per mounted `<Editor.svelte>`.** Do not recreate it on
   every prop change — use `view.dispatch({ changes })` to push new content.
3. **Keep the editor content in sync with the store**, but never let the store
   update fight the editor's own input. Pattern:
   - editor `updateListener` → debounce → write to store
   - store updates from elsewhere → `view.dispatch` (only if doc differs from
     current `view.state.doc`)
4. **Theme via a `EditorView.theme({...})` extension**, not by styling `.cm-*`
   classes with global CSS. Keep editor styling co-located in
   `src/lib/editor/theme.ts`.
5. **Phase 1 extensions** (only these):
   - `history()` + `keymap.of([...defaultKeymap, ...historyKeymap])`
   - `keymap.of(searchKeymap)` + `highlightSelectionMatches()`
   - `EditorView.lineWrapping` (journal entries wrap, never scroll-x)
   - `indentOnInput()` + `bracketMatching()` (cheap, slightly nice)
   - custom theme extension
6. **Do NOT add** `@codemirror/lang-markdown`, `@replit/codemirror-vim`,
   or `@codemirror/language`'s heavier features until Phase 2.

### Editor font + layout (non-negotiable)

```ts
EditorView.theme({
  '&': { height: '100%', backgroundColor: 'transparent' },
  '.cm-scroller': { fontFamily: 'inherit' },
  '.cm-content': {
    fontFamily: '"Newsreader", Georgia, serif',
    fontSize: '17px',
    lineHeight: '1.7',
    padding: '2rem 2.5rem',
    caretColor: '#1f2937',
  },
  '.cm-line': { padding: 0 },
  '.cm-gutters': { display: 'none' }, // no line numbers — this is a journal
})
```

No line numbers. No gutters. No active-line background highlight beyond a very
subtle one if at all. This is paper, not an IDE.

## Design system

### Background

Single solid white (`#ffffff`) in light mode, single solid `#1e1e1e` in dark.
No gradients, no noise textures. Theme follows the OS via `prefers-color-scheme`
(no manual toggle in Phase 1) — flips in one place: the `:root` tokens in `app.css`.

### Color palette (use sparingly)

| Token          | Hex       | Where                                     |
| -------------- | --------- | ----------------------------------------- |
| ink            | `#1f2937`  | journal body text, caret                  |
| ink-soft       | `#6b7280` | dates, secondary labels                   |
| ink-faint      | `#d1d5db` | checkbox outline, dividers                |
| accent         | `#3b82f6` | active toggle, checked-box fill, focus    |
| accent-soft    | `#eff6ff` | checked-box background tint               |
| paper          | `#ffffff` | app + editor background                   |

Define these as CSS custom properties in `app.css`. Do not hardcode hex values
throughout components — reference `var(--ink)` etc.

### Typography

- Journal body — Newsreader, 17px / 1.7 / weight 400. Italic available for emphasis.
- UI labels (habit names, switch) — Atkinson Hyperlegible, 14px, weight 400 (label)
  / 700 (active state).
- Date header (e.g. "Saturday, June 27") — Newsreader italic, 13px, `ink-soft`.

### Spacing

- App padding: `1.5rem` on all sides.
- Left/right split gap: `2rem` between journal pane and habit pane.
- Habit checkbox rows: `0.75rem` vertical rhythm.
- Journal editor padding: `2rem 2.5rem` inside the `.cm-content` (see theme above).

### Switch (Today/Monthly) — the only ornamented UI

- Pill shaped, full-radius, ~`9rem` wide, `2rem` tall.
- Centered horizontally at top of the window, `1.5rem` from the top edge.
- Left half label "Today", right half label "Monthly".
- Active side: `ink` text on `#f3f4f6` (very light grey) background.
  Inactive side: `ink-soft` text on transparent.
- The track transitions in a single ~150ms ease slide.
- The pill itself has a `1px` `ink-faint` outline, no drop shadow.
  Do not make this a glossy Apple-style switch — keep it flat and quiet.

### Habit checkbox

- `1.25rem` square (matches cap height of habit label).
- `1px` `ink-faint` border, `4px` radius. Not fully rounded — habit checkboxes
  are squares to read as "todo", not "radio".
- Checked: `accent` fill + white check icon (lucide `Check`, size 14).
- Label sits `0.625rem` to the right of the box, vertically centered.

## Modularity

This is a small app, but it will grow. Keep code broken into small, single-purpose
modules from day one — do not dump everything into `+page.svelte` or `lib.rs`.

### Frontend (Svelte/TS)

- **`+page.svelte` is a layout shell only.** It composes components and reads
  from stores. It should not contain editor logic, habit logic, or store wiring.
  If it grows past ~80 lines, extract something.
- **One concern per store file.** `journal.svelte.ts` only owns journal text.
  `habits.svelte.ts` only owns habit defs + checks. `view.svelte.ts` only owns
  the active view. Do not cross-wire concerns inside a store.
- **Editor concerns live in `src/lib/editor/`, not in `Editor.svelte`.**
  `extensions.ts` exports the Phase 1 extension preset.
  `theme.ts` exports the `EditorView.theme(...)` object.
  `Editor.svelte` imports them and wires lifecycle + store sync — nothing more.
- **Re-usable UI primitives** (the pill switch, the habit checkbox row, any
  future buttons) each get their own component file. Never inline markup for a
  repeated element across pages.
- **Cap component size at ~150 lines.** Hard rule. Above that, extract a
  sub-component or move logic to a store. The linter won't enforce it, you must.

### Rust (src-tauri)

- **`main.rs` stays tiny** — it calls into `lib.rs` and nothing else. Do not
  add commands to `main.rs`.
- **`lib.rs` is the app entry** (the `run()` function), but it should not hold
  command implementations. It registers modules and wires the Tauri builder.
- **Each distinct concern gets its own module file under `src/`.** For example,
  if Phase 2 adds a "backup export" command, create `src/backup.rs` and
  `mod backup;` in `lib.rs`. Do not pile commands into `lib.rs`.
- **A module is worth extracting when:** it has more than one function, or it
  has its own config/struct, or it's scoped to a feature (a window, a plugin,
  a file format). One-off helpers can stay inline.
- **Target Rust layout** (only create files you actually need):
  ```
  src-tauri/src/
    main.rs            # entry, calls diary_lib::run()
    lib.rs             # run() — builder, plugin registration, module wiring
    commands.rs        # #[tauri::command] functions, thin wrappers
    state.rs           # app state structs managed by Tauri
    <feature>.rs       # one file per distinct feature/concern
  ```
  Phase 1 will likely only need `main.rs` + `lib.rs`. Add `commands.rs` the
  moment you write your first `#[tauri::command]` — don't wait for a second.

### General principle

If a file can be named after what it *does* rather than where it *sits*, the
codebase stays navigable as it grows. A stranger reading the tree should be
able to guess what each file is for without opening it.

## Conventions

### Svelte components

- Always `<script lang="ts">`. No plain `<script>`.
- Use runes (`$state`, `$derived`, `$effect`, `$props`). Do not use the legacy
  reactive `$:` syntax.
- Props via `$props()` with TS types, no `export let`.
- One component per file, filename matches component name (PascalCase).
- Keep components small. If a component exceeds ~150 lines, extract sub-components
  or move logic to a store.

### TypeScript

- `strict: true` is already on. Do not loosen it.
- No `any` without a justification comment. Prefer `unknown` + type narrowing.
- Use `type` for plain shapes, `interface` only when you need declaration merging.
- Path alias: `$lib/...` is configured by SvelteKit. Use it.

### CSS

- Tailwind first. Use component-scoped `<style>` blocks only for things Tailwind
  can't express (e.g. the editor's `.cm-` overrides if absolutely necessary — but
  prefer the `EditorView.theme` route described above).
- No global `*` resets beyond what Tailwind's preflight already provides.

### Rust (src-tauri)

- Start minimal. Phase 1 may have zero Tauri commands — the plugin-store handles
  persistence purely on the JS side.
- Add a Tauri command only when something genuinely can't be done in JS
  (e.g. file dialogs, system integration). Document why in a comment above
  the command.
- Keep `Cargo.toml` deps lean. Do not add crates speculatively.

### Formatting

- 2-space indentation (matches existing files).
- Semicolons not required in Svelte/TS, but stay consistent within each file.
- Postfix types as `: Type` rather than prefix `Type =`.
- Import order: external → Svelte/Kit → `$lib/...` → relative → `*.svelte`
  last (Svelte components come after logic imports).

## Phase 2 / future work (do not build now, but design for)

Record these so future-you and future-agents don't accidentally build them.

- **Monthly view charts** — when building, use `layerchart` or
  `@svelte-plugins/echarts`. Monthly view is currently only a stub route.
- **Vim motions** — add `@replit/codemirror-vim` and a `vim()` extension.
  Toggle via a small settings button. Do not make vim the default.
- **Bullet lists in the editor** — add `@codemirror/lang-markdown` for
  bullet continuation and rendering. Markdown live-preview is **not** a goal;
  keep it plain-text feel with light auto-formatting only.
- **Dark mode** — already shipping in Phase 1 via `prefers-color-scheme`.
  A manual override toggle (light/dark/system) is a future polish item. Token
  system in `app.css` is already set up for it.
- **Cross-device sync** — explicitly not planned. Do not add cloud code paths.

## Files to never edit casually

- `src-tauri/tauri.conf.json` — only change when adding bundle config, window
  options, or new plugins. The 800x600 default window size will likely be
  increased; pick `width: 1200, height: 800` when you do.
- `svelte.config.js` — adapter-static + SPA fallback. Don't change the adapter.
- `vite.config.js` — the port `1420 strict` is load-bearing for Tauri.
- `src/routes/+layout.ts` — `ssr = false` is required for Tauri. Do not flip it.

## Commit message style

Short imperative summary, lowercase first word, no period:

- `stand up codemirror editor with debounced autosave`
- `add view switch pill with today/monthly toggle`
- `wire habit checkbox state to plugin-store`

If a change touches both frontend and `src-tauri/`, prefix with `t:` or `s:`
to disambiguate when useful. Otherwise no prefix.