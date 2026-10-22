# Diary

A native desktop notebook for daily writing and small habits, built in Rust with [GPUI Kit](https://gpui-kit.com/).

- Write today's plain-text journal, with wrapping, undo, selection, and copy/paste.
- Browse past entries through Yesterday or the calendar. Past notes are read-only.
- Track daily habits, rename or delete them, and keep a task backlog.
- Reflect on writing streaks, a year of writing activity, and habit completion.
- Follow the system's light or dark appearance. Fonts and icons are bundled for offline use.

The design comes from `Layout adjustment for maximized window-handoff.zip`. At 720px, habits and tasks sit above the note. From 1100px, they stack in a 300px sidebar beside the note. The minimum window is 720×500.

## Run

Install a current stable Rust toolchain. On Debian/Ubuntu, install GPUI's native build dependencies:

```bash
sudo apt-get install build-essential clang cmake pkg-config libasound2-dev \
  libfontconfig1-dev libfreetype-dev libxkbcommon-dev libxkbcommon-x11-dev \
  libwayland-dev libx11-xcb-dev libxcb1-dev libxcb-composite0-dev \
  libxcb-damage0-dev libxcb-dpms0-dev libxcb-present-dev libxcb-randr0-dev \
  libxcb-render0-dev libxcb-res0-dev libxcb-shape0-dev libxcb-sync-dev \
  libxcb-xfixes0-dev libxcb-xinput-dev libxcb-xkb-dev libvulkan-dev
cargo run --locked
```

Linux needs a working Vulkan graphics driver. macOS needs Xcode command line tools; Windows needs the Visual Studio C++ build tools and Windows SDK.

```bash
cargo fmt --check
cargo check --locked
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
```

GPUI is a large first build. `target/release/diary` is the Linux/macOS executable; Windows builds `diary.exe`. The tag release workflow packages a Linux archive, a macOS app, and a Windows ZIP. Packaging on macOS and Windows is verified by CI, not by a Linux build.

## Data

Diary retains the previous app's `com.khurram.diary/.diary-store.json` location under the OS data directory. Existing journal, habit, and task data loads directly, including older date-keyed tasks. Before the first save, Diary copies the original file to `.diary-store.pre-gpui.json` in the same directory.

Edits update memory immediately. A single background writer waits 400ms after the last change, writes a temporary file, then atomically replaces the store. Closing flushes pending writes. Load and save errors appear in the app; a damaged file is never silently replaced with an empty notebook.

For a separate notebook during development:

```bash
DIARY_DATA_DIR=/tmp/diary-demo cargo run --locked
```

Don't run the old Tauri app and this version against the same notebook simultaneously. Both applications save the entire file.

## Shortcuts

| Action | macOS | Linux / Windows |
| --- | --- | --- |
| Today | Cmd+T | Ctrl+T |
| Previous / next day | Cmd+[ / Cmd+] | Ctrl+[ / Ctrl+] |
| Save now | Cmd+S | Ctrl+S |
| Dismiss calendar/menu or cancel rename | Escape | Escape |

Enter adds a habit or task and commits a rename. Tab moves between controls. The view pill accepts left/right arrows. Completed tasks remain briefly available to undo by clicking their checkbox again.

## Updates

Settings checks the public GitHub releases API only when requested. A newer version opens its release page for manual download. This version does not replace its own executable or use the former Tauri updater.

### Upgrade from 0.1.x

Download 0.2.0 from the [GitHub release page](https://github.com/khurrambhutto/diary/releases/tag/v0.2.0) and install it manually. The former Tauri updater cannot install the native Rust app. Quit the old app before opening the new one. Diary reads the same notebook and creates `.diary-store.pre-gpui.json` before its first save.

Choose the Linux x86-64 archive, the macOS Apple Silicon or Intel app archive, or the Windows x86-64 ZIP. Extract the archive before running the app. On Linux, install the runtime libraries listed above and put `diary` on your `PATH` if using the included desktop file. On macOS, move `Diary.app` to Applications. The macOS app uses ad-hoc signing and is not notarized. On Windows, run `diary.exe` from the extracted folder.

Release downloads include `SHA256SUMS` for checking archive integrity.

### Publishing a release

Set the version in `Cargo.toml` and `Cargo.lock`, update the changelog, then push a matching `v` tag. The release workflow checks the tag against the package version, runs tests and Clippy, and builds Linux, macOS Apple Silicon, macOS Intel, and Windows archives. It creates a draft release with checksums. Verify the downloads and publish the draft when ready.

## Code

`src/main.rs` starts the app. `app.rs` owns UI state, `model.rs` owns data and statistics, and `storage.rs` owns disk writes. `inputs.rs` and `interactions.rs` connect editing and navigation to that state. Each screen lives in `src/ui/`; shared colors are in `theme.rs`. Fonts and SVG icons live in `assets/` with the font licenses.
