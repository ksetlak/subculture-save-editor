#!/bin/bash
set -e

echo "Building MacOS universal binary with podman..."

# Build the Docker image
podman build -f Dockerfile.macos -t subculture-macos-builder .

# Create output directory
mkdir -p dist

# Run the container and copy the binary
podman run --rm -v "$(pwd)/dist:/output:Z" subculture-macos-builder

echo "Universal binary created at: dist/subculture-save-editor-macos"
echo "Binary size: $(du -h dist/subculture-save-editor-macos | cut -f1)"