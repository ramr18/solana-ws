use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast::Receiver;

pub async fn start_websocket_server(rx: Receiver<String>, port: u16)
{
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("WebSocket server running on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let mut rx = rx.resubscribe();
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("WebSocket accept failed");
            let (mut write, _) = ws_stream.split();

            while let Ok(msg) = rx.recv().await {
                if write.send(tokio_tungstenite::tungstenite::Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        });
    }
}
