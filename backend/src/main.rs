use std::net::SocketAddr;

use stovoy_dev_backend_axum::build_router;
use tokio::signal;
use tracing::info;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Settings {
    #[serde(default = "default_bind")]
    bind: String,

    #[serde(default = "default_static_dir")]
    static_dir: String,
}

fn default_bind() -> String {
    "0.0.0.0:8080".to_owned()
}

fn default_static_dir() -> String {
    "dist".to_owned()
}

impl Settings {
    fn load() -> anyhow::Result<Self> {
        let mut cfg = config::Config::builder();
        cfg = cfg
            .add_source(config::File::with_name("settings").required(false))
            .add_source(config::Environment::with_prefix("APP").separator("__"));

        Ok(cfg.build()?.try_deserialize()?)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::load()?;

    let app = build_router(&settings.static_dir);

    let addr: SocketAddr = settings.bind.parse()?;

    info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm =
            signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
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
