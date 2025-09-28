use axum::Router;
use axum::routing::get;
use socketioxide::{SocketIo, extract::SocketRef};
use tracing_subscriber;

use server_rust::{
    models::{Lobby, Player},
    socket_router,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_ansi(true)
        .init();

    let (layer, io) = SocketIo::new_layer();

    // Register all socket routes in another file
    socket_router::register_socket_routes(&io);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
