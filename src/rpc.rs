use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn};
use serde_json::Value;
use tokio_tungstenite::connect_async;
use tokio::sync::broadcast::Sender;
use url::Url;
use crate::parser::parse_rpc_event;

/// Start RPC listener that connects to Solana RPC WebSocket
/// and broadcasts events to connected clients
/// 
/// # Arguments
/// * `tx` - Broadcast sender for event distribution
pub async fn start_rpc_listener(tx: Sender<String>) {
    let ws_url = std::env::var("RPC_WS")
        .unwrap_or_else(|_| {
            warn!("RPC_WS not set, using default");
            "wss://api.mainnet-beta.solana.com".to_string()
        });
    
    let program_id = std::env::var("PROGRAM_ID")
        .unwrap_or_else(|_| "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P".to_string());

    loop {
        match connect_and_listen(&ws_url, &program_id, tx.clone()).await {
            Ok(_) => {
                warn!("RPC connection closed, reconnecting...");
            }
            Err(e) => {
                error!("RPC connection error: {}, reconnecting in 5s...", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }
}

/// Connect to Solana RPC WebSocket and listen for events
async fn connect_and_listen(
    ws_url: &str,
    program_id: &str,
    tx: Sender<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(ws_url)?;
    let (ws_stream, _) = connect_async(url).await?;
    info!("Connected to Solana RPC WebSocket: {}", ws_url);

    let (mut write, mut read) = ws_stream.split();

    // Subscribe to logs for the program
    let sub_msg = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "logsSubscribe",
        "params": [
            { "mentions": [program_id] },
            { "commitment": "finalized" }
        ]
    });

    write.send(tungstenite::Message::Text(sub_msg.to_string()))
        .await?;
    info!("Subscribed to logs for program: {}", program_id);

    // Process incoming messages
    while let Some(msg) = read.next().await {
        match msg {
            Ok(tungstenite::Message::Text(text)) => {
                if let Ok(event_json) = serde_json::from_str::<Value>(&text) {
                    if let Some(event) = parse_rpc_event(&event_json) {
                        if let Ok(json_str) = serde_json::to_string(&event) {
                            if tx.send(json_str).is_err() {
                                error!("Failed to broadcast event: channel closed");
                                break;
                            }
                        }
                    }
                }
            }
            Ok(tungstenite::Message::Close(_)) => {
                info!("RPC connection closed by server");
                break;
            }
            Ok(_) => {
                // Ignore other message types
            }
            Err(e) => {
                error!("Error reading RPC message: {}", e);
                return Err(Box::new(e));
            }
        }
    }

    Ok(())
}
