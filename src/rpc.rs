use futures_util::{SinkExt};
use futures_util::StreamExt;
use serde_json::Value;
use tokio_tungstenite::connect_async;
use tokio::sync::broadcast::Sender;
use url::Url;
use crate::types::TokenEvent;

pub async fn start_rpc_listener(tx: Sender<String>) {
    let ws_url = std::env::var("RPC_WS").unwrap();
    let url = Url::parse(&ws_url).unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("WebSocket connection failed");
    println!("Connected to Solana RPC WebSocket");

    let (mut write, mut read) = ws_stream.split();

    let sub_msg = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "logsSubscribe",
        "params": [
            { "mentions": ["6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"] },
            { "commitment": "finalized" }
        ]
    });

    write.send(tungstenite::Message::Text(sub_msg.to_string()))
        .await
        .unwrap();

    while let Some(msg) = read.next().await {
        if let Ok(tungstenite::Message::Text(text)) = msg {
            let event_json: Value = serde_json::from_str(&text).unwrap();

            if let Some(logs) = event_json["params"]["result"]["value"]["logs"].as_array() {
                if logs.iter().any(|l| l.as_str().unwrap_or("").contains("initialize_token")) {
                    let event = TokenEvent {
                        event_type: "token_created".to_string(),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                        transaction_signature: event_json["params"]["result"]["value"]["signature"].as_str().unwrap_or("").to_string(),
                        token: super::types::TokenDetails {
                            mint_address: "Mint123...".to_string(),
                            name: "ExampleToken".to_string(),
                            symbol: "EXT".to_string(),
                            creator: "CreatorPubKey".to_string(),
                            supply: 1_000_000_000,
                            decimals: 6,
                        },
                        pump_data: super::types::PumpData {
                            bonding_curve: "BondingCurveAddr".to_string(),
                            virtual_sol_reserves: 30_000_000_000,
                            virtual_token_reserves: 1_073_000_000_000_000,
                        }
                    };

                    let json_str = serde_json::to_string(&event).unwrap();
                    let _ = tx.send(json_str);
                }
            }
        }
    }
}
