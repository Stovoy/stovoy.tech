use axum::routing::{get, get_service};
use axum::{extract::Path, response::IntoResponse, Router};
use stovoy_source::SOURCE_FILES;
use tower_http::services::ServeDir;

use once_cell::sync::OnceCell;

static STATIC_DIR: OnceCell<String> = OnceCell::new();

pub fn build_router(static_dir: &str) -> Router {
    let _ = STATIC_DIR.set(static_dir.to_owned());
    let static_service =
        get_service(ServeDir::new(static_dir).append_index_html_on_directories(true));
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/ping", get(ping))
        .route("/source/*path", get(source_handler))
        .route("/api/blogs", get(blogs_handler))
        .nest_service("/assets", static_service.clone())
        .fallback_service(static_service)
}

pub async fn healthz() -> &'static str {
    "ok"
}

pub async fn ping() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "message": "pong" }))
}

pub async fn source_handler(Path(raw_path): Path<String>) -> impl IntoResponse {
    let sanitized_path = {
        let segments = raw_path
            .split('/')
            .filter(|s| !s.is_empty() && *s != "..")
            .collect::<Vec<_>>();
        segments.join("/")
    };

    if let Some(content) = SOURCE_FILES.get(sanitized_path.as_str()) {
        ([("Content-Type", "text/plain; charset=utf-8")], *content).into_response()
    } else {
        axum::http::StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn blogs_handler() -> impl IntoResponse {
    use std::path::Path;
    let static_dir = STATIC_DIR.get().map(|s| s.as_str()).unwrap_or("dist");
    let path_root = Path::new(static_dir);
    let path1 = path_root.join("blogs.json");
    let content = std::fs::read_to_string(&path1).or_else(|_| {
        let p2 = path_root.join("blog/blogs.json");
        std::fs::read_to_string(p2)
    });

    let value: serde_json::Value = content
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(|| serde_json::json!([]));

    axum::Json(value)
}
