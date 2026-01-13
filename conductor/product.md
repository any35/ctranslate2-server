# Initial Concept
我想要利用ctranslate2-rs 把 ctranslate2 封装为http服务,最好支持openai api兼容接口. 然后写个docker文件, 以docker的方式提供服务(linux平台). 需要能提供常见的配置,如模型目录, gpu/cpu等. 支持nllb, t5, whisper

# Product Guide - ctranslate2-server

## Product Vision
A high-performance, production-ready Rust server that exposes CTranslate2 models via an OpenAI-compatible HTTP API. It aims to provide a drop-in replacement for OpenAI endpoints (Completions and Transcriptions) while offering the efficiency and flexibility of the CTranslate2 engine on both CPU and GPU hardware.

## Target Users
- Developers needing to self-host translation (NLLB/T5) or transcription (Whisper) models.
- Organizations seeking to reduce costs by migrating from OpenAI to open-source models without changing application code.
- DevOps engineers requiring a containerized, easily configurable inference server.

## Core Features
- **OpenAI Compatibility:** Implementation of `/v1/chat/completions` (for translation/text models) and `/v1/audio/transcriptions` (for Whisper).
- **Multi-Model Support:** Native support for NLLB, T5, and Whisper architectures via `ct2rs`.
- **Flexible Configuration:** Management of settings via Environment Variables, Configuration Files (TOML/YAML), CLI arguments, and a management API.
- **High Performance:** Designed for both high-throughput batch processing and low-latency real-time inference.
- **Dockerized Deployment:** Optimized multi-stage Docker builds with dedicated tags for CPU and GPU (CUDA) runtimes to minimize image size.

## Technical Goals
- **Framework:** Built with Axum for robust, asynchronous HTTP handling.
- **Hardware Optimization:** Support for NVIDIA GPU (CUDA) and CPU (MKL/DNNL) acceleration.
- **Reliability:** Built-in health checks for orchestration compatibility (Kubernetes/Docker Compose).
- **Scalability:** Support for dynamic model loading/unloading via API to manage memory efficiently.
