use crate::objects::states::SharedState;
pub mod helpers;
pub mod objects;
pub mod socket;
pub mod socket_router;
// pub mod responses;
pub mod responses;

pub async fn run_test_server(addr: &str, state: SharedState) {
    use axum::Router;
    use axum::routing::get;
    use socketioxide::SocketIo;

    let (layer, io) = SocketIo::new_layer();
    crate::socket_router::register_socket_routes(&io, &state);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // 👇 instead of .await, spawn in background
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
}
