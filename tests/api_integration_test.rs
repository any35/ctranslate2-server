use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use ctranslate2_server::{app, model::ModelManager, state::AppState};
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn chat_completions_returns_400_if_model_not_loaded() {
    let model_manager = Arc::new(ModelManager::new());
    let state = AppState { model_manager };
    let app = app(state);

    let request_body = json!({
        "model": "t5",
        "messages": [
            {"role": "user", "content": "Hello"}
        ]
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(body["error"].as_str().unwrap().contains("Model not loaded"));
}
