# Diary

Diary is an offline, single-user desktop journal and habit tracker. This branch replaces the former Tauri/Svelte app with native Rust and GPUI Kit 0.7.3. The previous stack remains in Git history.

## Before finishing

Run `cargo fmt --check`, `cargo check --locked`, `cargo test --locked`, and `cargo clippy --all-targets --locked -- -D warnings`. Fix errors and warnings in changed code. For a UI change, launch using an isolated `DIARY_DATA_DIR`; verify the compact and wide layouts and system appearance when the environment supports it. Report platform checks you could not perform.

Use 2-space Rust indentation, configured by `rustfmt.toml`. Keep entry-point wiring in `main.rs` and feature logic in named modules. Keep dependencies lean and commit `Cargo.lock`.

## Layout and ownership

- `src/app.rs`: mounted app state and initialization.
- `src/model.rs`: persisted data shapes, date keys, word counts, and streak calculations.
- `src/storage.rs`: compatibility reads, backup, serialized background saves, and flush.
- `src/inputs.rs`: input event subscriptions.
- `src/interactions.rs`: navigation, renaming, task completion, keyboard shortcuts.
- `src/ui/`: one module per screen or component concern; shared controls in `primitives.rs`.
- `src/theme.rs`: all shared design colors and font family names.
- `src/assets.rs`: embedded fonts and icons.
- `src/window_frame.rs`: transparent rounded window frame and resize edges.
- `src/ui/text_input.rs`: Base input presentation and tooltip text.
- `src/updates.rs`: explicit release checks.
- `assets/`: bundled fonts, licenses, and SVGs.
- `packaging/`: platform packaging assets.

## Design

Read the handoff ZIP's README and `project/DiaryApp.dc.html` before changing the design. Its `diary-tokens.css` is the visual reference. The current implementation deliberately uses the handoff's warm paper and lavender palette instead of the original white/blue theme.

Newsreader 16pt is the registered serif font family; use it for the 34px date heading and 17px journal body with 1.7 line height. Atkinson Hyperlegible is the 14px UI font. Bundle fonts locally.

The title bar is 36px and the top bar 48px. Compact content is centered with a 720px maximum and 40px horizontal padding. At 1100px window width, the date and note occupy the left column and Habits/To Do stack in a 300px right column with a 56px gap. Wide content has a 1240px maximum. Minimum window size is 720×500.

Initialize GPUI Kit Base and register `WindowFrame` on its Root so Diary owns the transparent, borderless window frame. Component initialization installs a square frame plugin, so use Base input controls with the app-owned presentation. Floating windows have 12px outer corners; maximized, tiled, and fullscreen windows have square corners.

Keep the page flat and quiet. Use 12px card radii, hairline borders, square habit checkboxes, round task checkboxes, and visible keyboard focus. Journal editing is plain text, without line numbers, syntax highlighting, or a toolbar. Dark mode follows the OS. Preserve selection/copy in read-only past notes.

## Persistence

Use `DIARY_DATA_DIR` for test/dev data. Never use the user's live notebook in automated tests.

Keep the existing OS data directory `com.khurram.diary/.diary-store.json`. Journal keys are `YYYY-MM-DD`. Habit checks are keyed by habit ID, then day. Todos are one global backlog. Preserve unknown store keys and support the previous date-keyed todo shape.

Update in-memory journal text on every edit, tagged with the selected date. Disk writes are debounced by 400ms on one background writer. Switching days must preserve pending text under its original date. Flush on close and report failures before discarding the window. Writes must be atomic, and the first GPUI save must retain the pre-rewrite backup. A failed or malformed load must never trigger a default-data overwrite.

## Scope

Keep offline journaling independent of release checks. Check GitHub only on explicit user action. Updates currently open the release page for manual download; keep labels accurate. No accounts or cloud sync. Markdown and Vim editing are future work.

Commit messages use a short imperative summary, lowercase first word, no final period.
