use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://127.0.0.1:8085").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();

    // Send a message to the server
    write.send(Message::Text("Hello, world!".into())).await.expect("Failed to send message");

    // Read and print messages from the server
    while let Some(msg) = read.next().await {
        let msg = msg.expect("Error reading message");
        if msg.is_text() || msg.is_binary() {
            println!("Received: {}", msg.to_text().unwrap());
        }
    }
}
