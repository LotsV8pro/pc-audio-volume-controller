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
  <a href="#-build">Build</a> •
  <a href="#-roadmap">Roadmap</a> •
  <a href="#-usage">Usage</a>
</p>

---

## ◈ Features

- **Real-time PipeWire node discovery** — detects all audio sinks, sources, and stream nodes
- **Visual connection graph** — see all ports and their routing at a glance
- **Per-node volume control** — individual sliders for every audio node
- **Remove connections** — click ✕ to disconnect any route
- **Lotus Purple theme** — matches the Lotus-Arch desktop aesthetic

## ◈ Build

```bash
# Dependencies
sudo pacman -S pipewire

# Build & run
git clone https://github.com/LotsV8pro/pc-audio-volume-controller.git
cd pc-audio-volume-controller
cargo run --release
```

Requires Rust 1.70+ and a running PipeWire session.

## ◈ Roadmap

- [x] PipeWire node discovery (`pw-cli` + `pw-link`)
- [x] Connection graph viewer
- [x] Per-node volume sliders
- [ ] Drag-and-drop port connections
- [ ] Create/remove links from the UI
- [ ] Real-time monitoring with `pw-link -m`
- [ ] Per-application EQ profiles
- [ ] Virtual device creation (loopback, combined sinks)
- [ ] Save/load routing presets
- [ ] System tray integration
- [ ] Keyboard shortcuts

---

<p align="center">
  <a href="https://github.com/LotsV8pro/Lotus-Arch"><img src="https://img.shields.io/badge/Lotus_Arch-141218?style=flat-square&logo=arch-linux&logoColor=C4A8E2"/></a>
  <a href="https://github.com/LotsV8pro/lotus-spotify"><img src="https://img.shields.io/badge/Lotus_Spotify-141218?style=flat-square&logo=spotify&logoColor=C4A8E2"/></a>
  <a href="https://github.com/LotsV8pro/lotus-discord"><img src="https://img.shields.io/badge/Lotus_Discord-141218?style=flat-square&logo=discord&logoColor=C4A8E2"/></a>
</p>

<p align="center">
  <sub>MIT License — <a href="https://github.com/LotsV8pro">@LotsV8pro</a></sub>
</p>
