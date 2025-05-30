use std::net::SocketAddr;

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::{connect_async, tungstenite::Message};

async fn spawn_app() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let app = stovoy_dev_backend_axum::build_router("dist");

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    addr
}

#[tokio::test]
async fn broadcast_reaches_all_clients() {
    let addr = spawn_app().await;
    let url = format!("ws://{}{}", addr, "/api/game/arena");

    let (mut client_a, _) = connect_async(&url).await.unwrap();
    let (mut client_b, _) = connect_async(&url).await.unwrap();

    let payload = "hello from a";
    client_a.send(Message::Text(payload.into())).await.unwrap();

    let received = tokio::time::timeout(std::time::Duration::from_secs(1), client_b.next())
        .await
        .expect("timed out")
        .expect("stream closed")
        .unwrap();

    assert_eq!(received, Message::Text(payload.into()));
}
