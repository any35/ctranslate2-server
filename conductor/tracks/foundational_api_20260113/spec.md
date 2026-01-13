# Track Specification: Foundational API and Model Infrastructure

## 1. Overview
This track establishes the core infrastructure for the `ctranslate2-server`. It focuses on setting up the Axum web server, implementing a robust configuration system, creating a thread-safe wrapper for the CTranslate2 engine (via `ct2rs`), and delivering a functional `/v1/chat/completions` endpoint compatible with the OpenAI API. It also includes Dockerizing the application for both CPU and GPU environments.

## 2. Goals
- **Core Server:** Initialize a production-ready Axum server with `tracing` and `snafu`.
- **Configuration:** Implement a hierarchical configuration system (Default < Config File < Env Vars < CLI Args).
- **Inference Engine:** Create a high-performance, concurrent wrapper for `ct2rs` to handle NLLB and T5 models.
- **OpenAI API:** Implement the `/v1/chat/completions` endpoint with correct request/response schemas.
- **Dockerization:** Create multi-stage Dockerfiles for optimized CPU and GPU deployment.

## 3. Key Features
- **Config Management:** Support for `config.toml`, `RUST_LOG`, `MODEL_PATH`, etc.
- **Model Management:** Lazy loading or startup loading of CTranslate2 models.
- **Chat Completion:** Basic text-to-text generation mapping OpenAI messages to model inputs.
- **Health Checks:** `/health` endpoint for Kubernetes liveness/readiness.

## 4. Technical Design
- **Server:** `axum` with `tokio` runtime.
- **State:** `Arc<AppState>` containing the `ModelManager`.
- **Model Wrapper:** `ModelManager` struct wrapping `ct2rs::Translator` in `Arc<Mutex<...>>` or `tokio::sync::Mutex` (depending on `ct2rs` thread-safety). *Note: `ct2rs` translators are generally thread-safe, but we need to verify if internal locking is needed.*
- **API Schema:** Use `serde` structs to match OpenAI's `ChatCompletionRequest` and `ChatCompletionResponse`.
- **Error Handling:** `snafu` for domain-specific errors (e.g., `ModelLoadError`, `InferenceError`), mapped to OpenAI JSON error responses.

## 5. Non-Functional Requirements
- **Performance:** Minimal overhead in the HTTP layer.
- **Concurrency:** Ensure the model wrapper does not block the async runtime unexpectedly (use `spawn_blocking` for heavy inference if needed).
- **Observability:** Full request tracing with correlation IDs.

## 6. API Endpoints
- `GET /health`: Returns 200 OK.
- `POST /v1/chat/completions`: Accepts OpenAI Chat JSON, returns JSON response.
