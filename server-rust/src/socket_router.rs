use crate::{
    socket::lobby_socket::{LobbyPayload, lobby_controller},
    state::SharedState,
};
use colored::Colorize;
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};
use tracing::warn;

pub fn register_socket_routes(io: &socketioxide::SocketIo, state: &SharedState) {
    let state = state.clone();
    io.ns("/", move |s: SocketRef| {
        let state = state.clone();
        println!("Client connected with id: {}", s.id.to_string().green());

        s.on("message", |s: SocketRef| {
            s.emit("message-back", "Hello World!").ok();
        });

        s.on(
            "lobbyControl",
            move |socket: SocketRef, Data::<LobbyPayload>(data)| {
                lobby_controller(socket, data, state.clone());
            },
        );
    });
}
