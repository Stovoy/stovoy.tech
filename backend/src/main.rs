use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize structured logging. Respects RUST_LOG, defaults to info.
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Shared broadcast channel for arena chat.
    // Capacity is small; if lagging clients cannot keep up they will drop messages.
    let (tx, _rx) = tokio::sync::broadcast::channel::<String>(128);

    // Build application routes and inject shared state.
    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/api/game/arena", get(arena_ws_handler))
        .layer(Extension(tx));

    // Bind address from $BIND or default.
    let addr: SocketAddr = std::env::var("BIND")
        .unwrap_or_else(|_| "0.0.0.0:8080".into())
        .parse()?;

    info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn healthz() -> &'static str {
    "ok"
}

async fn arena_ws_handler(
    ws: WebSocketUpgrade,
    Extension(tx): Extension<tokio::sync::broadcast::Sender<String>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx))
}

async fn handle_socket(mut socket: WebSocket, tx: tokio::sync::broadcast::Sender<String>) {
    // Subscribe to the broadcast channel to receive messages from other clients.
    let mut rx = tx.subscribe();

    loop {
        tokio::select! {
            // Receive message from WebSocket client
            from_client = socket.recv() => {
                match from_client {
                    Some(Ok(Message::Text(text))) => {
                        // Broadcast to all listeners; ignore error if there are no receivers.
                        let _ = tx.send(text);
                    }
                    Some(Ok(Message::Binary(_))) => {
                        // Ignore binary frames for now.
                    }
                    Some(Ok(Message::Ping(p))) => {
                        if socket.send(Message::Pong(p)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) => {
                        break;
                    }
                    Some(Ok(Message::Pong(_))) => {}
                    Some(Err(_)) | None => {
                        // Client disconnected or error.
                        break;
                    }
                }
            }

            // Receive broadcasted message from other client(s).
            from_broadcast = rx.recv() => {
                match from_broadcast {
                    Ok(msg) => {
                        if socket.send(Message::Text(msg)).await.is_err() {
                            break;
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                        // Skip missed messages.
                        continue;
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        }
    }
}

async fn shutdown_signal() {
    // Wait for SIGINT or SIGTERM.
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        sigterm.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
