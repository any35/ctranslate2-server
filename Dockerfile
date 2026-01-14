# Stage 1: Runtime (CPU)
FROM debian:bookworm-slim AS runtime-cpu

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libgomp1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy locally built binary (copied to root by build script)
COPY ctranslate2-server /app/server

# Default configuration
ENV RUST_LOG=info
EXPOSE 8080

ENTRYPOINT ["/app/server"]

# Stage 2: Runtime (GPU)
FROM nvidia/cuda:12.2.0-base-ubuntu22.04 AS runtime-gpu

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libgomp1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy locally built binary (copied to root by build script)
COPY ctranslate2-server /app/server

ENV RUST_LOG=info
EXPOSE 8080

ENTRYPOINT ["/app/server"]