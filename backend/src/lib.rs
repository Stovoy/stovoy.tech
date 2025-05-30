use axum::routing::{get, get_service};
use axum::Router;
use tower_http::services::ServeDir;

pub fn build_router(static_dir: &str) -> Router {
    let static_service = get_service(ServeDir::new(static_dir).append_index_html_on_directories(true));
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/ping", get(ping))
        .nest_service("/assets", static_service.clone())
        .fallback_service(static_service)
}

pub async fn healthz() -> &'static str {
    "ok"
}

pub async fn ping() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "message": "pong" }))
}

// Arena websocket functionality removed.
