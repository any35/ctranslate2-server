# Stage 1: Builder
FROM rust:1.82-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    g++ \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Build the application
COPY src ./src
# Update the timestamp of main.rs to ensure it's rebuilt
RUN touch src/main.rs
RUN cargo build --release

# Stage 2: Runtime (CPU)
FROM debian:bookworm-slim AS runtime-cpu

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/ctranslate2-server /app/server

# Default configuration
ENV RUST_LOG=info
EXPOSE 8080

ENTRYPOINT ["/app/server"]

# Stage 3: Runtime (GPU)
# Note: For GPU, we'd use an nvidia/cuda base image
FROM nvidia/cuda:12.2.0-base-ubuntu22.04 AS runtime-gpu

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/ctranslate2-server /app/server

ENV RUST_LOG=info
EXPOSE 8080

ENTRYPOINT ["/app/server"]
