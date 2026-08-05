# Changelog

## 0.1.5 — 2026-08-05

- Todo and habit labels wrap to a second line instead of overflowing
- Compact centered writing bars on the analytics page
- Traffic-light window control colors now appear on hover only

## 0.1.4 — 2026-07-25

- Add auto-update via Tauri updater plugin (native update dialog on startup)
- Built-in update checking: silent on launch, manual button in Settings
- Signing key pair generated for secure update artifact verification
- Release workflow now signs builds and publishes update.json manifest
- Add @tauri-apps/plugin-updater and @tauri-apps/plugin-process dependencies

## 0.1.3 — 2026-07-24

- New app icon branding across desktop app bundles, favicons, and settings
- Today view design makeover: refined card hairlines, elevation, and live status badges
- Apple Reminders style hollow ring checkboxes for todos
- Improved habit row options button hover visibility and smooth input styling
- Default window size set to 720×1000

## 0.1.2 — 2026-07-21

- Fix window size to 800×1000 for a taller, book-like layout
- Fix Tailwind v4 Vite plugin crash on startup

## 0.1.1 — 2026-07-21

- Add calendar popover to navigate past entries
- Click date heading to jump back to today
- Page swipe transitions (forward/back)
- Persist open todos across days with age badge
- Habit streak pill matches todo badge style with colored 🔥 emoji
- Auto-capitalize first letter of todos and habits
- Fix calendar popover clipping (positioning and layout)

## 0.1.0 — 2026-07-20

- Initial release
- Daily journal editor (CodeMirror)
- Habit tracking with checkboxes and streaks
- Todo list per day
- Three-segment nav pill (Yesterday / Today / Analytics)
- Dark mode support (follows OS)
- Offline-first with Tauri plugin-store
