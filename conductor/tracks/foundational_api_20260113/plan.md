# Track Plan: Foundational API and Model Infrastructure

## Phase 1: Core Setup & Configuration [checkpoint: 66e9446]
Establish the project skeleton, logging, and configuration management.

- [x] Task: Initialize Axum Server & Tracing [56c5a61]
    - [x] Subtask: Add dependencies (`axum`, `tokio`, `tracing`, `tracing-subscriber`).
    - [x] Subtask: Write tests for a basic `/health` endpoint.
    - [x] Subtask: Implement `main.rs` to setup `tracing` and start `axum` server.
- [x] Task: Implement Configuration System [eede5b4]
    - [x] Subtask: Add `config`, `serde`, `clap` dependencies.
    - [x] Subtask: Define `AppConfig` struct.
    - [x] Subtask: Write tests for loading config from defaults, file, env, and args.
    - [x] Subtask: Implement config loading logic.
- [x] Task: Conductor - User Manual Verification 'Core Setup & Configuration' (Protocol in workflow.md) [66e9446]

## Phase 2: CTranslate2 Model Wrapper [checkpoint: 90d767f]
Implement the safe Rust wrapper for the inference engine.

- [x] Task: Implement ModelManager Struct [2e1fbcf]
    - [x] Subtask: Add `ct2rs` dependency.
    - [x] Subtask: Define `ModelManager` and `ModelType` enum (NLLB, T5, Whisper).
    - [x] Subtask: Write integration tests for loading a dummy/small model.
    - [x] Subtask: Implement `ModelManager::new()` and `load_model()`.
- [x] Task: Implement Inference Logic (Text Generation) [7000603]
    - [x] Subtask: Define trait/method for `predict` or `chat`.
    - [x] Subtask: Write tests for generating text from a loaded model.
    - [x] Subtask: Implement `generate` method using `ct2rs` APIs, wrapping blocking calls in `spawn_blocking` if necessary.
- [x] Task: Conductor - User Manual Verification 'CTranslate2 Model Wrapper' (Protocol in workflow.md) [90d767f]

## Phase 3: OpenAI API Implementation [checkpoint: 5c84454]
Build the HTTP layer compatible with OpenAI.

- [x] Task: Define OpenAI Request/Response Schemas [479c7cf]
    - [x] Subtask: Create `api/openai.rs` module.
    - [x] Subtask: Define structs: `ChatCompletionRequest`, `ChatCompletionMessage`, `ChatCompletionResponse`, `ChatCompletionChoice`.
    - [x] Subtask: Write serialization/deserialization tests to ensure compatibility with official OpenAI examples.
- [x] Task: Implement /v1/chat/completions Endpoint [a73e92b]
    - [x] Subtask: Write integration test using `axum::test_helpers` or `reqwest` against the mock server.
    - [x] Subtask: Implement the handler function connecting `Axum` -> `ModelManager`.
    - [x] Subtask: Handle errors using `snafu` and map to OpenAI error format.
- [x] Task: Conductor - User Manual Verification 'OpenAI API Implementation' (Protocol in workflow.md) [5c84454]

## Phase 4: Dockerization & Final Polish
Prepare for deployment.

- [x] Task: Create Dockerfile [9ecc333]
    - [x] Subtask: Create `.dockerignore`.
    - [x] Subtask: Write multi-stage `Dockerfile` with targets for `cpu` and `gpu`.
    - [x] Subtask: Verify build size and layer caching.
- [~] Task: Conductor - User Manual Verification 'Dockerization & Final Polish' (Protocol in workflow.md)
