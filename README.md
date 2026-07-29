<p align="center">
  <img src="https://img.shields.io/badge/Electron-141218?style=for-the-badge&logo=electron&logoColor=C4A8E2"/>
  <img src="https://img.shields.io/badge/Audio_Mixer-141218?style=for-the-badge&logo=pipewire&logoColor=C4A8E2"/>
  <img src="https://img.shields.io/badge/WIP-141218?style=for-the-badge&logo=codeforces&logoColor=C4A8E2"/>
</p>

<h1 align="center">◈ PC AUDIO VOLUME CONTROLLER ◈</h1>

<p align="center">
  <b>Visual per-application audio mixer — Sonar-like interface for Linux</b><br>
  <sub>Electron app · PipeWire integration · Keyboard + GUI control</sub>
</p>

<p align="center">
  <a href="#-features">Features</a> •
  <a href="#-roadmap">Roadmap</a> •
  <a href="#-install">Install</a> •
  <a href="#-usage">Usage</a>
</p>

---

## ◈ Features

- Per-application volume sliders (Music, Browser, System, Games)
- Keyboard shortcuts (F13–F22) for quick adjustments
- Mute/unmute all with a single key
- Reset all volumes to 50%
- Electron GUI with real-time slider feedback

## ◈ Roadmap

The goal is to evolve this into a **Sonar-like visual audio routing matrix**:

- [x] Per-app volume sliders
- [ ] Real-time PipeWire node detection (all active audio streams)
- [ ] Drag-and-drop audio routing between apps and outputs
- [ ] Visual connection graph (like `pw-link` but graphical)
- [ ] Per-application EQ profiles
- [ ] Virtual device creation (loopback, combined sinks)
- [ ] System tray integration
- [ ] Save/load routing presets

## ◈ Install

```bash
git clone https://github.com/LotsV8pro/pc-audio-volume-controller.git
cd pc-audio-volume-controller
npm install
npm start
```

A web demo is also available at `web-version.html`.

## ◈ Usage

### GUI

Launch the app to see sliders for each configured audio source.

### Keyboard Shortcuts

| Key | Action |
|---|---|
| `F13` / `F14` | Music Player volume ± |
| `F15` / `F16` | Browser volume ± |
| `F17` / `F18` | System Sounds volume ± |
| `F19` / `F20` | Games volume ± |
| `F21` | Mute/unmute all |
| `F22` | Reset all to 50% |

---

<p align="center">
  <sub>MIT License — <a href="https://github.com/LotsV8pro">@LotsV8pro</a></sub>
</p>
