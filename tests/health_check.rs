use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use ctranslate2_server::{app, config::AppConfig, model::ModelManager, state::AppState};
use std::sync::Arc;
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn health_check_works() {
    let model_manager = Arc::new(ModelManager::new(AppConfig::default()));
    let state = AppState { model_manager };
    let app = app(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
