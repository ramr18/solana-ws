mod error;
mod parser;
mod rpc;
mod server;
mod types;

use dotenv::dotenv;
use log::{error, info};
use std::env;
use tokio::sync::broadcast;

/// Main entry point for the Solana WebSocket Service
#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logger
    env_logger::init();
    
    info!("Starting Solana WebSocket Service");

    // Get configuration from environment
    let port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "9000".to_string())
        .parse()
        .unwrap_or_else(|_| {
            error!("Invalid SERVER_PORT, using default 9000");
            9000
        });

    // Create broadcast channel for event distribution
    let (tx, rx) = broadcast::channel::<String>(100);
    info!("Created broadcast channel with capacity: 100");

    // Spawn RPC listener task
    let rpc_tx = tx.clone();
    tokio::spawn(async move {
        rpc::start_rpc_listener(rpc_tx).await;
    });

    // Start WebSocket server (this blocks)
    server::start_websocket_server(rx, port).await;
}
