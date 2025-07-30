mod rpc;
mod server;
mod types;
use dotenv::dotenv;
use std::env;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let port: u16 = env::var("SERVER_PORT").unwrap_or("9000".into()).parse().unwrap();
    let (tx, rx) = broadcast::channel::<String>(100);

    tokio::spawn(rpc::start_rpc_listener(tx.clone()));
    server::start_websocket_server(rx, port).await;
}
