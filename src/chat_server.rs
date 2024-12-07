// chat_server.rs
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

type Tx = mpsc::UnboundedSender<Result<Message, warp::Error>>;

#[derive(Clone)]
pub struct ChatServer {
    clients: Arc<Mutex<HashMap<String, Tx>>>,
}

impl ChatServer {
    pub fn new() -> Self {
        ChatServer {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn handle_connection(&self, ws: WebSocket, client_id: String) {
        let (mut user_ws_tx, mut user_ws_rx) = ws.split();
        let (tx, rx) = mpsc::unbounded_channel();
        let mut rx = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

        self.clients.lock().unwrap().insert(client_id.clone(), tx);

        tokio::spawn(async move {
            while let Some(message) = rx.next().await {
                if let Ok(msg) = message {
                    if user_ws_tx.send(msg).await.is_err() {
                        break;
                    }
                }
            }
        });

        while let Some(result) = user_ws_rx.next().await {
            if let Ok(msg) = result {
                if msg.is_text() {
                    self.broadcast_message(msg.to_str().unwrap(), client_id.clone())
                        .await;
                }
            }
        }

        self.clients.lock().unwrap().remove(&client_id);
    }

    pub async fn broadcast_message(&self, message: &str, sender_id: String) {
        let clients = self.clients.lock().unwrap();
        for (client_id, client) in clients.iter() {
            if client_id != &sender_id {
                let _ = client.send(Ok(Message::text(message)));
                // println!("Broadcasting message to client {}: {}", client_id, message);
                println!("Broadcasting message to client {}: {}", client_id, message);
            }
        }
    }
}
