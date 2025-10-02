use std::sync::Arc;

use crate::{
    socket::{
        disconnect_controller,
        lobby_socket::{LobbyPayload, lobby_controller},
    },
    state::SharedState,
};
use colored::Colorize;
use serde_json::Value;
use socketioxide::{
    extract::{Data, SocketRef},
    socket::DisconnectReason, SocketIo,
};
use tracing::warn;

pub fn register_socket_routes(io: &socketioxide::SocketIo, state: &SharedState) {
    let state = state.clone();
    io.ns("/", move |s: SocketRef| {
        println!("Client connected with id: {}", s.id.to_string().green());

        s.on("message", |s: SocketRef| {
            s.emit("message-back", "Hello World!").ok();
        });

        let state_for_lobby = state.clone();
        s.on("lobbyControl", move |socket: SocketRef, Data::<LobbyPayload>(data)| {
            lobby_controller(socket, data, state_for_lobby.clone());
        });

        let state_for_disconnect = state.clone();
        s.on_disconnect(move |socket: SocketRef, reason: DisconnectReason| {
            disconnect_controller(socket, reason, state_for_disconnect.clone());
        });
    });
}
