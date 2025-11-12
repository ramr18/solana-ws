use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::sync::broadcast;
use log::{info, error};

pub async fn start_ws_server(tx: broadcast::Sender<String>, port: u16) {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await.expect("Can't bind");
    info!("WebSocket server listening on ws://{}", addr);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let mut rx = tx.subscribe();
                tokio::spawn(async move {
                    match accept_async(stream).await {
                        Ok(ws_stream) => {
                            let (mut sink, _) = ws_stream.split();
                            while let Ok(msg) = rx.recv().await {
                                if let Err(e) = sink.send(Message::Text(msg)).await {
                                    error!("Client send error: {:?}", e);
                                    break;
                                }
                            }
                        }
                        Err(e) => error!("WS accept error: {:?}", e),
                    }
                });
            }
            Err(e) => error!("Accept error: {:?}", e),
        }
    }
}
