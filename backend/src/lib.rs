use axum::extract::ws::Message;
use axum::extract::{ws::WebSocketUpgrade, Extension};
use axum::response::IntoResponse;
use axum::routing::{get, get_service};
use axum::{extract::ws::WebSocket, Router};
use tower_http::services::{ServeDir, ServeFile};

pub fn build_router(static_dir: &str) -> Router {
    let (tx, _rx) = tokio::sync::broadcast::channel::<String>(128);
    let static_service = get_service(ServeDir::new(static_dir));
    let index_service = get_service(ServeFile::new(format!("{}/index.html", static_dir)));
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/ping", get(ping))
        .route("/api/game/arena", get(arena_ws_handler))
        .nest_service("/assets", static_service.clone())
        .fallback_service(index_service)
        .layer(Extension(tx))
}

pub async fn healthz() -> &'static str {
    "ok"
}

pub async fn ping() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "message": "pong" }))
}

pub async fn arena_ws_handler(
    ws: WebSocketUpgrade,
    Extension(tx): Extension<tokio::sync::broadcast::Sender<String>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx))
}

async fn handle_socket(mut socket: WebSocket, tx: tokio::sync::broadcast::Sender<String>) {
    let mut rx = tx.subscribe();
    loop {
        tokio::select! {
            from_client = socket.recv() => {
                match from_client {
                    Some(Ok(Message::Text(text))) => {
                        let _ = tx.send(text);
                    }
                    Some(Ok(Message::Binary(_))) => {}
                    Some(Ok(Message::Ping(p))) => {
                        if socket.send(Message::Pong(p)).await.is_err() { break; }
                    }
                    Some(Ok(Message::Pong(_))) => {}
                    Some(Ok(Message::Close(_))) => { break; }
                    Some(Err(_)) | None => { break; }
                }
            }
            from_broadcast = rx.recv() => {
                match from_broadcast {
                    Ok(msg) => {
                        if socket.send(Message::Text(msg)).await.is_err() { break; }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(_) => break,
                }
            }
        }
    }
}
