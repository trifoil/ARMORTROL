use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};

async fn handle_connection(stream: TcpStream) {
    let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");
    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        let msg = msg.expect("Error reading message");
        if msg.is_text() || msg.is_binary() {
            write.send(msg).await.expect("Error sending message");
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8085".to_string();
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("Server listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}