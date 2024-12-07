mod auth;
mod chat_server;
mod log;
mod routes;

use auth::Auth;
use chat_server::ChatServer;
use log::init_logger;
use routes::routes;
use std::sync::Arc;
use tokio::signal;
use warp::Filter;

#[tokio::main]
async fn main() {
    let chat_server = Arc::new(ChatServer::new());
    let auth = Arc::new(Auth::new("secret_key".to_string()));

    init_logger();

    // let app_routes = routes(chat_server.clone())
    let app_routes = routes(chat_server.clone(), auth.clone())
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
