#!/bin/bash
set -e

echo "Building Linux binary with podman..."

# Build the Docker image
podman build -f Dockerfile.linux-static -t subculture-builder .

# Create output directory
mkdir -p dist

# Run the container and copy the binary
podman run --rm -v "$(pwd)/dist:/output:Z" subculture-builder

echo "Binary created at: dist/subculture-save-editor"
echo "Binary size: $(du -h dist/subculture-save-editor | cut -f1)"
echo "Dependencies: $(ldd dist/subculture-save-editor | wc -l) shared libraries"