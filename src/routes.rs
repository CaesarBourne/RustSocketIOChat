// use crate::chat_server::ChatServer;
// use std::sync::Arc;
// use warp::ws::Ws;
// use warp::Filter;

// pub fn routes(
//     chat_server: Arc<ChatServer>,
// ) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path("ws")
//         .and(warp::ws())
//         .and(warp::any().map(move || Arc::clone(&chat_server)))
//         .map(|ws: Ws, chat_server: Arc<ChatServer>| {
//             ws.on_upgrade(move |socket| async move {
//                 let chat_server_clone = Arc::clone(&chat_server);
//                 chat_server_clone.handle_connection(socket).await;
//             })
//         })
// }

// routes.rs
use crate::chat_server::ChatServer;
use std::sync::Arc;
use warp::ws::Ws;
use warp::Filter;

pub fn routes(
    chat_server: Arc<ChatServer>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param::<String>())
        .and(warp::any().map(move || Arc::clone(&chat_server)))
        .map(|ws: Ws, client_id: String, chat_server: Arc<ChatServer>| {
            ws.on_upgrade(move |socket| async move {
                let chat_server_clone = Arc::clone(&chat_server);
                chat_server_clone.handle_connection(socket, client_id).await;
            })
        })
}
