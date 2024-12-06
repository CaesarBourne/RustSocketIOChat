mod chat_server;
mod routes;

use chat_server::ChatServer;
use routes::routes;
use std::sync::Arc;
use tokio::signal;
use warp::Filter;

#[tokio::main]
async fn main() {
    let chat_server = Arc::new(ChatServer::new());

    let app_routes = routes(chat_server.clone())
        .or(warp::path("health").map(|| warp::reply::json(&"Server is running")));
    println!("Server is running at http://127.0.0.1:3030");
    let server = warp::serve(app_routes).run(([127, 0, 0, 1], 3030));

    tokio::select! {
        _ = server => {},
        _ = signal::ctrl_c() => {
            println!("Shutting down server...");
        }
    }
}
