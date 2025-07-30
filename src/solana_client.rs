use anyhow::Result;
use futures::{SinkExt, StreamExt};
use log::{error, info};
use serde_json::json;
use tokio::sync::broadcast;
use tokio_tungstenite::connect_async;
use crate::pump_parser::parse_pump_tx;

pub async fn start_solana_listener(tx: broadcast::Sender<String>, rpc_ws_url: &str) -> Result<()> {
    loop {
        match connect_async(rpc_ws_url).await {
            Ok((ws_stream, _)) => {
                info!("Connected to Solana WebSocket {}", rpc_ws_url);
                let (mut write, mut read) = ws_stream.split();

                let subscribe_msg = json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "logsSubscribe",
                    "params": [
                        { "mentions": ["6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"] },
                        { "commitment": "finalized" }
                    ]
                });

                if let Err(e) = write.send(tokio_tungstenite::tungstenite::Message::Text(subscribe_msg.to_string())).await {
                    error!("Subscribe error: {:?}", e);
                    continue;
                }

                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(m) => {
                            if let tokio_tungstenite::tungstenite::Message::Text(txt) = m {
                                if let Some(event) = parse_pump_tx("sample_sig", &txt) {
                                    if let Ok(json_str) = serde_json::to_string(&event) {
                                        let _ = tx.send(json_str);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            error!("Read error: {:?}, reconnecting...", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                error!("Connection failed: {:?}, retrying...", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}
