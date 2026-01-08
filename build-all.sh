#!/bin/bash
set -e

echo "Building for all targets..."

# Windows
echo "Building Windows binary..."
cargo build --target x86_64-pc-windows-gnu --release

# Linux static
echo "Building Linux static binary..."
./build-linux-static.sh

# MacOS universal
echo "Building MacOS universal binary..."
./build-macos.sh

echo "All builds completed!"
echo "Outputs:"
echo "  Windows: target/x86_64-pc-windows-gnu/release/subculture-save-editor.exe"
echo "  Linux:   dist/subculture-save-editor"
echo "  MacOS:   dist/subculture-save-editor-macos"