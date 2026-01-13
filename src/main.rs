use ctranslate2_server::{app, config::{AppConfig, Args}};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = AppConfig::load(Some(args)).expect("Failed to load configuration");

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ctranslate2_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .expect("Invalid address");
    
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}