use axum::{routing::get, Router};

pub mod config;

pub fn app() -> Router {
    Router::new().route("/health", get(health_check))
}

async fn health_check() -> &'static str {
    "OK"
}
