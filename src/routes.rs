// routes.rs
use crate::auth::Auth;
use crate::chat_server::ChatServer;
use std::sync::Arc;
use warp::ws::Ws;
use warp::Filter;

pub fn routes(
    chat_server: Arc<ChatServer>,
    auth: Arc<Auth>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param::<String>())
        .and(warp::header("Authorization"))
        .and(warp::any().map(move || Arc::clone(&chat_server)))
        .and(warp::any().map(move || Arc::clone(&auth)))
        .map(
            |ws: Ws,
             client_id: String,
             token: String,
             chat_server: Arc<ChatServer>,
             auth: Arc<Auth>| {
                if auth.verify_token(&token) {
                    Box::new(ws.on_upgrade(move |socket| async move {
                        let chat_server_clone = Arc::clone(&chat_server);
                        chat_server_clone.handle_connection(socket, client_id).await;
                    })) as Box<dyn warp::Reply>
                } else {
                    Box::new(warp::reply::with_status(
                        warp::reply::json(&"Invalid token"),
                        warp::http::StatusCode::UNAUTHORIZED,
                    )) as Box<dyn warp::Reply>
                }
            },
        )
}
