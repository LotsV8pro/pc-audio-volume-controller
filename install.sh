#!/usr/bin/env bash
set -euo pipefail

PREFIX="${PREFIX:-/usr/local}"

if ! command -v cargo &>/dev/null; then
    echo "Error: Rust (cargo) is required. Install it from https://rustup.rs"
    exit 1
fi

if ! command -v pw-cli &>/dev/null; then
    echo "Warning: pw-cli not found — make sure PipeWire is installed and running."
fi

if ! command -v wpctl &>/dev/null; then
    echo "Error: wpctl not found — install WirePlumber (pipewire-wireplumber)."
    exit 1
fi

echo "→ Building lotus-audio (release)..."
cargo build --release

echo "→ Installing to ${PREFIX}/bin..."
sudo install -Dm755 target/release/lotus-audio "${PREFIX}/bin/lotus-audio"

echo "→ Installing desktop entry..."
sudo install -Dm644 lotus-audio.desktop "${PREFIX}/share/applications/lotus-audio.desktop"

echo "→ Installing icon..."
sudo install -Dm644 icons/hicolor/scalable/apps/lotus-audio.svg "${PREFIX}/share/icons/hicolor/scalable/apps/lotus-audio.svg"

if command -v update-desktop-database &>/dev/null; then
    echo "→ Updating desktop database..."
    sudo update-desktop-database "${PREFIX}/share/applications" 2>/dev/null || true
    sudo gtk-update-icon-cache -f -t "${PREFIX}/share/icons/hicolor" 2>/dev/null || true
fi

echo "✓ Done. Run: lotus-audio"
