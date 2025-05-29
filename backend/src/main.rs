use std::net::SocketAddr;

use stovoy_tech_backend_axum::build_router;
use tokio::signal;
use tracing::info;

// Observability – OpenTelemetry exporter + tracing bridge
use opentelemetry::global;
use opentelemetry::sdk::trace as sdktrace;
use tracing_subscriber::prelude::*;

// --- Configuration --------------------------------------------------------

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Settings {
    /// TCP bind address, e.g. "0.0.0.0:8080".
    #[serde(default = "default_bind")]
    bind: String,

    /// Directory to serve static assets from.
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
        // Priority:
        //   1. Environment variables (PREFIX_*)
        //   2. settings.{yaml,toml,env}
        //   3. Defaults from struct

        let mut cfg = config::Config::builder();

        // Optional: a `settings.yaml` near the binary / CWD.
        cfg = cfg
            .add_source(config::File::with_name("settings").required(false))
            // Environment variables in the form `APP_BIND=0.0.0.0:9000`
            .add_source(config::Environment::with_prefix("APP").separator("__"));

        Ok(cfg.build()?.try_deserialize()?)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing & OpenTelemetry exporter. RUST_LOG controls filter.
    let exporter = opentelemetry_stdout::SpanExporter::default();
    let provider = sdktrace::TracerProvider::builder()
        .with_simple_exporter(exporter)
        .build();

    global::set_tracer_provider(provider);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_opentelemetry::layer())
        .init();

    // Load configuration (env, file, defaults).
    let settings = Settings::load()?;

    // Shared broadcast channel for arena chat.
    // Capacity is small; if lagging clients cannot keep up they will drop messages.
    let app = build_router(&settings.static_dir);

    // Use configured bind address.
    let addr: SocketAddr = settings.bind.parse()?;

    info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    // Ensure all spans are exported before exit.
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}

async fn shutdown_signal() {
    // Wait for SIGINT or SIGTERM.
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
