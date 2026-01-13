pub mod api;
pub mod config;
pub mod model;
pub mod state;

use axum::{Router, routing::get, routing::post};
use state::AppState;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/v1/chat/completions", post(api::openai::chat_completions))
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}
