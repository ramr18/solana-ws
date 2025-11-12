use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast::Receiver;
use log::{error, info, warn};

/// Start WebSocket server that broadcasts events to connected clients
/// 
/// # Arguments
/// * `rx` - Broadcast receiver for event distribution
/// * `port` - Port to listen on
pub async fn start_websocket_server(rx: Receiver<String>, port: u16) {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    info!("WebSocket server running on ws://{}", addr);

    while let Ok((stream, peer_addr)) = listener.accept().await {
        let mut client_rx = rx.resubscribe();
        info!("New client connected from: {}", peer_addr);
        
        tokio::spawn(async move {
            match accept_async(stream).await {
                Ok(ws_stream) => {
                    let (mut write, _) = ws_stream.split();
                    
                    while let Ok(msg) = client_rx.recv().await {
                        if write
                            .send(tokio_tungstenite::tungstenite::Message::Text(msg))
                            .await
                            .is_err()
                        {
                            warn!("Client {} disconnected", peer_addr);
                            break;
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to accept WebSocket connection from {}: {}", peer_addr, e);
                }
            }
        });
    }
}
