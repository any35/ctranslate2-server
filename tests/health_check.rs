use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use ctranslate2_server::app;
use tower::ServiceExt; // for `oneshot` // I'll need to make this available

#[tokio::test]
async fn health_check_works() {
    let app = app();

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
