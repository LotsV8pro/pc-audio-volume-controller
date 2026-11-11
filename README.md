<p align="center">
  <img src="https://img.shields.io/badge/Rust-141218?style=for-the-badge&logo=rust&logoColor=C4A8E2"/>
  <img src="https://img.shields.io/badge/egui-141218?style=for-the-badge&logo=codeforces&logoColor=C4A8E2"/>
  <img src="https://img.shields.io/badge/PipeWire-141218?style=for-the-badge&logo=pipewire&logoColor=C4A8E2"/>
</p>

<h1 align="center">Lotus Audio</h1>

A small window that shows the PipeWire audio graph and lets you route each
application to an output and set its volume, in the style of the SteelSeries
Sonar mixer. Written in Rust with egui.

The window has one column per output (a zone). Inside each column are the
applications currently playing into it, and a vertical fader for the zone
volume. Dragging an application from one column to another reconnects its
streams.

## What it does

- Reads the graph with `pw-cli` and `pw-link` and shows sinks, sources and the
  streams linked to them.
- Shows a fader per zone, and remembers the volume of each one across refreshes.
- Moves an application to another output when you drop its pill on that column.
- Lists streams that are not linked to any zone, so a forgotten application does
  not disappear from the mixer.
- Lets you hide the zones you do not use from the settings panel.
- Refresh is manual or automatic, on a two-second timer.

The graph is queried by calling the PipeWire command line tools, so the program
does not link against libpipewire and does not need the headers to build.

## Install

```bash
git clone https://github.com/LotsV8pro/pc-audio-volume-controller.git
cd pc-audio-volume-controller
./install.sh
```

The script builds the release binary and installs `lotus-audio`, the desktop
entry and the icon under `/usr/local`, so it asks for the sudo password.

To build without installing:

```bash
cargo build --release
./target/release/lotus-audio
```

## Requirements

- A running PipeWire session with WirePlumber (`wpctl` is required, the script
  stops if it is missing)
- `pw-cli` and `pw-link`, from the `pipewire` package
- Rust 1.70 or newer, to build

## Use

Run `lotus-audio`. Drag a pill to move an application, drag a fader to change a
zone volume, and use the gear button to choose which zones are visible. The
refresh button re-reads the graph; the auto-refresh toggle does it every two
seconds.

Configuration is stored in `~/.config/lotus-audio/config.json`: the list of
enabled sinks and the saved volumes.

## Platform

PipeWire is the audio server on Linux, so the program is Linux only. A Windows or
macOS port would need a different backend (WASAPI, CoreAudio) and the routing
code would have to be rewritten around it. There is no plan for that.

## Related repos

- [Lotus-Arch](https://github.com/LotsV8pro/Lotus-Arch): the desktop it was written for
- [lotus-mic-chain](https://github.com/LotsV8pro/lotus-mic-chain): the microphone chain that feeds this graph

## License

MIT.
