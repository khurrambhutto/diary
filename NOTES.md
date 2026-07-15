# Overnight Build — Session Summary

Branch: `overnight/phase-2-build`
Completed: 2026-07-15

## What was built

### 1. Editor cleaned up — pure prose surface
- Removed `bracketMatching()`, `indentOnInput()`, `highlightSelectionMatches()`, and `searchKeymap` extensions
- Removed `@codemirror/search` and `@codemirror/language` imports entirely
- The editor is now a true blank page: history + line-wrapping + custom theme only
- Added `flushSave()` on blur so content isn't lost if user navigates away before the 400ms debounce fires

### 2. To-do list (ephemeral, per-day)
- New store: `src/lib/stores/todos.svelte.ts`
- Persisted in plugin-store under `todos` key, keyed by `YYYY-MM-DD`
- Adding a task inserts it; clicking the circle removes it immediately (no archive)
- On a new day, the list starts fresh (ephemeral by design — keyed to today's date)
- Components: `TodoList.svelte`, `TodoRow.svelte`
- Empty state: "Nothing for today" in italic `--ink-soft`

### 3. Habits extended — delete + streaks
- `deleteHabit(id)` — removes habit def + all its check state, persists immediately
- `habitStreak(id)` — counts consecutive checked days backwards from yesterday; today is counted only if already checked (standard streak convention)
- Streak displayed inline (e.g. "5🔥"), always visible when > 0
- Delete button (× icon) appears on hover, turns red on hover
- `HabitAddForm.svelte` — inline input with + button, follows the same border-bottom-input pattern as the todo form
- Empty state: "No habits yet" in italic

### 4. Right panel redesigned
- 70/30 split maintained (journal left, sidebar right)
- Sidebar is a scrollable column: Habits section → hairline divider → To-Do section
- Both sections use `text-transform: uppercase` section headers matching the premium muted-label convention
- Section headers: "HABITS" and "TO DO" in Atkinson Hyperlegible 14px 700 weight

### 5. Design polish
- Todo check circle: on hover, a checkmark fades in with scale animation (120ms ease)
- Habit checkbox: color transition 150ms on check/uncheck
- Streak badge: always visible, not hover-dependent
- Delete buttons: hidden until row hover (opacity 0→1, 150ms)
- All inputs share the same border-bottom pattern with `--accent` focus color
- Empty states use italic `--ink-soft` at 13px
- All colors reference CSS variables — dark mode works automatically via `prefers-color-scheme`

## Verification

- `bun run check`: 0 errors, 0 warnings
- `cargo check` (src-tauri): clean
- No `console.log` or debug code left in

## What's left (out of scope for this session)

- Monthly view is still a Phase 2 stub — the "More" button on the ViewSwitch navigates to the placeholder page
- No dark mode manual toggle (follows OS only, per existing design)
- No vim motions or bullet-list auto-formatting

## Commits (chronological)

1. `750f9ed` — strip code-ide extensions from journal editor, add blur save
2. `7e340e6` — add todo store and components, extend habits with streak/delete, redesign right panel
3. `4591cb6` — polish: always show streak, animate todo check icon, add empty states
4. `493b6de` — fix streak calculation: start from yesterday, count today only when checked
