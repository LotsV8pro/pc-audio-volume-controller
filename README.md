<p align="center">
  <img src="https://img.shields.io/badge/Rust-141218?style=for-the-badge&logo=rust&logoColor=C4A8E2"/>
  <img src="https://img.shields.io/badge/egui-141218?style=for-the-badge&logo=codeforces&logoColor=C4A8E2"/>
  <img src="https://img.shields.io/badge/PipeWire-141218?style=for-the-badge&logo=pipewire&logoColor=C4A8E2"/>
</p>

<h1 align="center">◈ LOTUS AUDIO ◈</h1>

<p align="center">
  <b>Visual PipeWire audio router — Sonar-like interface for Linux</b><br>
  <sub>Rust + egui · real-time node graph · drag-and-drop routing</sub>
</p>

<p align="center">
  <a href="#-features">Features</a> •
  <a href="#-install">Install</a> •
  <a href="#-usage">Usage</a> •
  <a href="#-build-from-source">Build</a>
</p>

---

## ◈ Features

- **Real-time PipeWire node discovery** — detects all audio sinks, sources, and stream nodes
- **Per-zone volume control** — vertical faders for every output zone
- **Drag-and-drop routing** — drag any app pill onto a zone column to route it there
- **Disconnected stream monitor** — shows orphaned streams not linked to any zone
- **Zone visibility toggles** — show/hide zones from the settings panel
- **Volume persistence** — volume levels survive graph refreshes
- **Lotus Purple theme** — dark aesthetic matching Lotus-Arch

## ◈ Install

```bash
git clone https://github.com/LotsV8pro/pc-audio-volume-controller.git
cd pc-audio-volume-controller
./install.sh
```

Or manually:

```bash
cargo build --release
sudo install target/release/lotus-audio /usr/local/bin/
sudo install lotus-audio.desktop /usr/local/share/applications/
```

### Dependencies

- A running **PipeWire** session (pipewire, pipewire-pulse)
- **Rust** 1.70+ (`rustc`, `cargo`)

## ◈ Usage

```bash
lotus-audio
```

- **Route an app** — drag a pill (app name) from any zone column and drop it onto another zone
- **Adjust volume** — drag the vertical slider in any zone column
- **Toggle zones** — click the ⚙ button in the top bar, check/uncheck zones
- **Refresh** — click ↻ or enable Auto-refresh toggle

## ◈ Build from source

```bash
git clone https://github.com/LotsV8pro/pc-audio-volume-controller.git
cd pc-audio-volume-controller
cargo build --release
```

Requires Rust 1.70+ and a running PipeWire session. PipeWire libraries are accessed at runtime via `pw-cli` and `pw-link` — no build-time dependency.

## ◈ Windows / macOS

PipeWire is a Linux audio server. This app **will not work** on Windows or macOS without a complete audio backend replacement (WASAPI / CoreAudio). A cross-platform backend is not currently planned.

---

<p align="center">
  <a href="https://github.com/LotsV8pro/Lotus-Arch"><img src="https://img.shields.io/badge/Lotus_Arch-141218?style=flat-square&logo=arch-linux&logoColor=C4A8E2"/></a>
  <a href="https://github.com/LotsV8pro/lotus-spotify"><img src="https://img.shields.io/badge/Lotus_Spotify-141218?style=flat-square&logo=spotify&logoColor=C4A8E2"/></a>
  <a href="https://github.com/LotsV8pro/lotus-discord"><img src="https://img.shields.io/badge/Lotus_Discord-141218?style=flat-square&logo=discord&logoColor=C4A8E2"/></a>
</p>

<p align="center">
  <sub>MIT License — <a href="https://github.com/LotsV8pro">@LotsV8pro</a></sub>
</p>
