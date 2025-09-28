use colored::*;
use socketioxide::{extract::SocketRef, socket::Socket};
use tracing::info;

/// Handles all incoming socket routes
pub fn register_socket_routes(io: &socketioxide::SocketIo) {
    io.ns("/", |s: SocketRef| {
        println!("Client connected with id: {}", s.id.to_string().green());

        s.on("message", |s: SocketRef| {
            s.emit("message-back", "Hello World123!").ok();
        });
    });
}

/// Example: lobby handler
fn handle_lobby(socket: Socket, data: String) {
    // TODO: forward to lobby logic
    socket
        .emit("lobby_response", format!("Joined lobby with {data}"))
        .ok();
}

/// Example: game handler
fn handle_game(socket: Socket, data: String) {
    // TODO: forward to game instance logic
    socket
        .emit("game_response", format!("Joined game with {data}"))
        .ok();
}
