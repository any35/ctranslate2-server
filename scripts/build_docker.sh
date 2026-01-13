#!/bin/bash
set -e

IMAGE_NAME="any35/ctranslate2-server"

echo "Building Docker image: $IMAGE_NAME"

# Build Rust binary locally
echo "Compiling binary (release)..."
cargo build --release

# Build CPU version
echo "Building Docker image (CPU)..."
docker build --target runtime-cpu -t "${IMAGE_NAME}:latest" -t "${IMAGE_NAME}:cpu" .

# Optional: Build GPU version
# echo "Building Docker image (GPU)..."
# docker build --target runtime-gpu -t "${IMAGE_NAME}:gpu" .

echo "Build complete!"
