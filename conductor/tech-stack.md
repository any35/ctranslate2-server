# Technology Stack - ctranslate2-server

## Core Backend
- **Language:** Rust (Edition 2024) - Selected for memory safety and high performance.
- **HTTP Framework:** [Axum](https://github.com/tokio-rs/axum) - An async-first web framework that integrates seamlessly with the Tokio ecosystem.
- **Async Runtime:** [Tokio](https://tokio.rs/) - The industry-standard asynchronous runtime for Rust.

## Machine Learning & Inference
- **Inference Engine:** [CTranslate2](https://github.com/OpenNMT/CTranslate2) - A fast inference engine for Transformer models.
- **Rust Bindings:** [`ct2rs`](https://crates.io/crates/ct2rs) - Efficient Rust bindings for the CTranslate2 C++ library.
- **Supported Architectures:** NLLB, T5, and Whisper.

## API & Data Handling
- **API Specification:** OpenAI-compatible REST API.
- **Serialization:** [Serde](https://serde.rs/) - High-performance framework for serializing and deserializing Rust data structures.
- **Documentation:** [OpenAPI / Swagger](https://swagger.io/specification/) - Provided via integration with `utoipa` for interactive API documentation.

## Infrastructure & Deployment
- **Containerization:** Docker - Using multi-stage builds to optimize image size.
- **Runtimes:**
    - **CPU:** Optimized with MKL/DNNL.
    - **GPU:** NVIDIA CUDA support.
- **Configuration:** Managed via Environment Variables, TOML/YAML files, and CLI arguments using crates like `config` and `clap`.

## Observability & Error Handling
- **Logging:** [`tracing`](https://github.com/tokio-rs/tracing) - For structured, asynchronous logging. Must be configured to include source code locations (file, line) and spans to visualize the call chain.
- **Error Handling:**
    - **Application Errors:** [`snafu`](https://github.com/shepmaster/snafu) - For defining structured, strongly-typed domain errors with context.
    - **Ad-hoc Errors:** [`anyhow`](https://github.com/dtolnay/anyhow) - For easy error propagation in non-library code or main application logic.
    - **Requirement:** All error logs must include location data and full backtraces/causal chains to facilitate debugging.
- **Health Checks:** Native Axum endpoints for container orchestration readiness/liveness probes.
