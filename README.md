# t3code-desktop-rune

T3 Code Desktop recreated in [Rune](https://github.com/anomalyco/rune) (Rust GPU UI) for Linux — 1:1 with T3 Code Desktop Nightly.

Parity target: T3 Code `0.1.7-nightly.20261102.4061` (`t3 --version`). See [PARITY.md](PARITY.md).

## Status

Early scaffold (day 1): app shell (SplitView sidebar + toolbar), thread list,
Markdown transcript, TextArea composer (local echo only), diff placeholder,
command palette stub (⌘K), settings stub. Headless snapshots render on a
display-less Linux server via Lavapipe. Everything that touches the server is
**read-only** (`t3 thread list --json`, `t3 thread show --json`); the app never
sends prompts and never reads tokens.

## Rune dependency

Until the needed pieces land upstream, this repo pins the `t3/desktop-linux`
branch of the rune fork (same rev review policy as upstream `dev`):

```toml
rune = { git = "https://github.com/ImBIOS/rune", branch = "t3/desktop-linux" }
```

`Cargo.lock` pins the exact rev. Contributions made to deliver this go
upstream first; the fork branch is a waiting room, not a fork-away.

## Run

Prereqs (Ubuntu 24.04, headless OK):

```sh
sudo apt-get install -y libxkbcommon-dev libwayland-dev libxkbcommon-x11-0 mesa-vulkan-drivers ffmpeg
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json  # GPU-less render
```

```sh
cargo run -p t3-desktop                      # window (needs a display)
cargo run -p t3-desktop -- --list-scenes
cargo run -p t3-desktop -- --scene session --snap session.png
cargo run -p t3-desktop -- --scene session --script scripts/click-refresh.json --frames 60 --out frames/
```

## Layout

- `crates/t3-desktop/src/main.rs` — AppSpec (title, size, scenes, fonts, keys)
- `crates/t3-desktop/src/model.rs` — routes, threads, messages, composer state
- `crates/t3-desktop/src/t3rpc.rs` — read-only `t3 thread list|show --json` bridge
- `crates/t3-desktop/src/views.rs` — shell, sidebar, transcript, composer, palette
- `scripts/` — headless input scripts; `snaps/` — golden snapshots
