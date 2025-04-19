use std::net::SocketAddr;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use tracing::{info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize structured logging. Respects RUST_LOG, defaults to info.
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Build application routes.
    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/api/game/arena", get(arena_ws_handler));

    // Bind address from $BIND or default.
    let addr: SocketAddr = std::env::var("BIND")
        .unwrap_or_else(|_| "0.0.0.0:8080".into())
        .parse()?;

    info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn healthz() -> &'static str {
    "ok"
}

async fn arena_ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // Basic echo for now.
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                if socket.send(Message::Text(text)).await.is_err() {
                    return;
                }
            }
            Message::Binary(bin) => {
                if socket.send(Message::Binary(bin)).await.is_err() {
                    return;
                }
            }
            Message::Ping(p) => {
                let _ = socket.send(Message::Pong(p)).await;
            }
            Message::Close(_) | Message::Pong(_) => {}
        }
    }
}
