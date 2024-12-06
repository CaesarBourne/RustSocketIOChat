use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use warp::ws::{Message, WebSocket};

#[tokio::main]
async fn main() {
    let token = auth::generate_token("username");
    let mut stream = TcpStream::connect("localhost:3030").await.unwrap();
    let mut ws = WebSocket::new(stream);

    ws.send(Message::text("Hello, server!")).await.unwrap();

    while let Some(message) = ws.next().await {
        if let Ok(msg) = message {
            if msg.is_text() {
                println!("Received message: {}", msg.to_str().unwrap());
            }
        }
    }
}
