// Minimal Arceon Server - Reduced dependencies
use anyhow::Result;
use clap::Parser;
use tracing::{info, error};
use std::sync::Arc;
use warp::Filter;
use serde_json::json;

// Only essential modules
use arceon_core::{ArceonCore, Config};

#[derive(Parser)]
#[command(author, version, about = "Minimal Arceon Game Server")]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Server port
    #[arg(short, long, default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();
    
    info!("Starting Minimal Arceon Server on port {}", cli.port);
    
    // Initialize core
    let config = Config::load(&cli.config)?;
    let core = Arc::new(ArceonCore::new(config)?);
    
    // Health check endpoint
    let health = warp::path("health")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&json!({
                "status": "ok",
                "service": "arceon-mini-server",
                "version": "0.1.0"
            }))
        });
    
    // Basic API info
    let api_info = warp::path("api")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&json!({
                "message": "Minimal Arceon Server API",
                "endpoints": [
                    "/health - Health check",
                    "/api - API information"
                ]
            }))
        });
    
    // Combine routes
    let routes = health.or(api_info);
    
    info!("Minimal Arceon Server started successfully on port {}", cli.port);
    
    // Start server
    warp::serve(routes)
        .run(([127, 0, 0, 1], cli.port))
        .await;
    
    Ok(())
}