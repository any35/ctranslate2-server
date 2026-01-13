#!/bin/bash
set -e

IMAGE_NAME="any35/ctranslate2-server"

echo "Building Docker image: $IMAGE_NAME"

# Build CPU version
docker build --target runtime-cpu -t "${IMAGE_NAME}:latest" -t "${IMAGE_NAME}:cpu" .

# Optional: Build GPU version
docker build --target runtime-gpu -t "${IMAGE_NAME}:gpu" .

echo "Build complete!"
