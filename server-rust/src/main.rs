use std::sync::{Arc, Mutex};

use axum::{http};
use axum::routing::get;
use socketioxide::SocketIo;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

use server_rust::{
    socket_router,
    state::ServerState,
    state::SharedState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_ansi(true)
        .init();

    let state: SharedState = Arc::new(Mutex::new(ServerState::new()));

    let (layer, io) = SocketIo::new_layer();

    // Register all socket routes in another file
    socket_router::register_socket_routes(&io, &state);

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<http::HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3100").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
