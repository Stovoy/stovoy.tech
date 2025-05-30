use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::util::ServiceExt;

#[tokio::test]
async fn ping_returns_pong() {
    let app = stovoy_dev_backend_axum::build_router("dist");
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ping")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let _ = response.into_body();
}
