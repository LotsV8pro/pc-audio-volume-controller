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

echo "→ Building lotus-audio (release)..."
cargo build --release

echo "→ Installing to ${PREFIX}/bin..."
sudo install -Dm755 target/release/lotus-audio "${PREFIX}/bin/lotus-audio"

echo "→ Installing desktop entry..."
sudo install -Dm644 lotus-audio.desktop "${PREFIX}/share/applications/lotus-audio.desktop"

echo "✓ Done. Run: lotus-audio"
