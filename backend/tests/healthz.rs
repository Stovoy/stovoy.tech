use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::util::ServiceExt;
use stovoy_dev_backend_axum::build_router;

#[tokio::test]
async fn healthz_ok() {
    let app = build_router("dist");
    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
