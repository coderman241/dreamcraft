# DreamCraft

An open-source, Dreamweaver-style visual web editor, written in Rust with [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui/tree/master/crates/eframe).

**Not Adobe Dreamweaver, and not affiliated with Adobe.** This is a small, from-scratch, MIT-licensed editor inspired by the idea of editing code and seeing a preview side by side. No Adobe code, assets, or trademarks are used.

## What it does (v0.1.0)

- Code pane on the left, structure preview on the right
- Insert heading, paragraph, link, image, list, and button snippets
- Open / Save `.html` files by typing a path in the File box
- A simple tag inspector that counts how many times a tag appears
- New document starter template

The preview is a structure preview, not a full browser engine. For a pixel-perfect render, open the saved `.html` file in a browser.

## Build and run

You need Rust (rustup is the usual way to get it). The app is pure
Rust + egui, with no platform-specific code, so the same source builds
on Linux and on Windows.

### Linux

```sh
cargo run
```

Or build a release binary:

```sh
cargo build --release
./target/release/dreamcraft
```

On Debian/Ubuntu you may first need the windowing libraries egui
depends on:

```sh
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```

### Windows

Install Rust from https://rustup.rs/ (the MSVC toolchain), then in
PowerShell or Command Prompt, from the project folder:

```powershell
cargo run
cargo build --release
.\target\release\dreamcraft.exe
```

### Cross-compile Windows from Linux (optional)

With MinGW installed on Linux, the included `.cargo/config.toml`
points the Windows GNU target at the MinGW linker:

```sh
rustup target add x86_64-pc-windows-gnu
sudo apt-get install gcc-mingw-w64-x86-64
cargo build --release --target x86_64-pc-windows-gnu
# output: target/x86_64-pc-windows-gnu/release/dreamcraft.exe
```

GitHub Actions (`.github/workflows/ci.yml`) also builds release
binaries on both `ubuntu-latest` and `windows-latest` on every push.

## Project layout

```
dreamcraft/
  Cargo.toml
  src/main.rs        # the whole app: code editor, preview, inspector
  assets/logo.png    # DreamCraft logo (original artwork)
  .cargo/config.toml # MinGW linker for cross-compiling to Windows from Linux
  .github/workflows/ci.yml  # release builds on Linux and Windows
  README.md
  LICENSE            # MIT
```

## Roadmap ideas

- Real cursor-position inserts in the code pane
- Syntax highlighting
- A true HTML renderer / live browser preview
- Split view, file tree, and project folders
- CSS editing pane

Contributions welcome — this is meant to stay small and hackable.
