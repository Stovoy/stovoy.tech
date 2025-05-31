use axum::routing::{get, get_service};
use axum::Router;
use axum::{extract::Path, response::IntoResponse};
use stovoy_source::SOURCE_FILES;
use tower_http::services::ServeDir;

pub fn build_router(static_dir: &str) -> Router {
    let static_service =
        get_service(ServeDir::new(static_dir).append_index_html_on_directories(true));
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/ping", get(ping))
        .route("/source/*path", get(source_handler))
        .nest_service("/assets", static_service.clone())
        .fallback_service(static_service)
}

pub async fn healthz() -> &'static str {
    "ok"
}

pub async fn ping() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "message": "pong" }))
}

pub async fn source_handler(Path(path): Path<String>) -> impl IntoResponse {
    if let Some(content) = SOURCE_FILES.get(path.as_str()) {
        ([("Content-Type", "text/plain; charset=utf-8")], *content).into_response()
    } else {
        axum::http::StatusCode::NOT_FOUND.into_response()
    }
}
