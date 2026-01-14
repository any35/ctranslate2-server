#!/bin/bash
set -e

IMAGE_NAME="any35/ctranslate2-server"
PUSH=false

while getopts "p" opt; do
  case $opt in
    p)
      PUSH=true
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      exit 1
      ;;
  esac
done

echo "Building Docker image: $IMAGE_NAME"

# Build Rust binary locally
echo "Compiling binary (release)..."
cargo build --release

# Copy binary to root context to respect .dockerignore
cp target/release/ctranslate2-server ./ctranslate2-server

# Build CPU version
echo "Building Docker image (CPU)..."
docker build --target runtime-cpu -t "${IMAGE_NAME}:latest" -t "${IMAGE_NAME}:cpu" .

# Build GPU version
# echo "Building Docker image (GPU)..."
docker build --target runtime-gpu -t "${IMAGE_NAME}:gpu" .

# Cleanup
rm ./ctranslate2-server

echo "Build complete!"

if [ "$PUSH" = true ]; then
  echo "Pushing images to Docker Hub..."
  docker push "${IMAGE_NAME}:latest"
  docker push "${IMAGE_NAME}:cpu"
  docker push "${IMAGE_NAME}:gpu"
  echo "Push complete!"
fi
