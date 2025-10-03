
use crate::{
    socket::{
        disconnect_controller,
        lobby_socket::{lobby_controller, LobbyPayload}, start_game_controller, start_game_socket::StartGamePayload,
    },
    state::SharedState,
};
use colored::Colorize;
use socketioxide::{
    extract::{Data, SocketRef},
    socket::DisconnectReason, SocketIo,
};


pub fn register_socket_routes(io: &SocketIo, state: &SharedState) {
    let state = state.clone();
    let io_inside = io.clone();
    io.ns("/", move |s: SocketRef| {
        println!("Client connected with id: {}", s.id.to_string().green());

        s.on("message", |s: SocketRef| {
            s.emit("message-back", "Hello World!").ok();
        });

        let state_for_debug = state.clone();
        s.on("debug", move |s: SocketRef| {
            s.emit("message-back", format!("{:?}", state_for_debug.clone())).ok();
        });

        let state_for_lobby = state.clone();
        s.on("lobbyControl", |socket: SocketRef, Data::<LobbyPayload>(data)| {
            lobby_controller(socket, data, state_for_lobby);
        });

        let state_for_start_game = state.clone();
        s.on("startGame", |socket: SocketRef, Data::<StartGamePayload>(data)| {
            start_game_controller(socket, data, state_for_start_game);
        });

        let state_for_disconnect = state.clone();
        s.on_disconnect(|socket: SocketRef, reason: DisconnectReason| {
            disconnect_controller(socket, reason, state_for_disconnect, io_inside);
        });
    });
}
