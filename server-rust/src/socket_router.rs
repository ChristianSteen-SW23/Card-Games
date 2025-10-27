use crate::{
    objects::states::SharedState,
    socket::{
        disconnect_controller, lobby_socket::{LobbyPayload, lobby_controller}, start_game_controller, start_game_socket::StartGamePayload
    },
};
use colored::Colorize;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef},
    socket::DisconnectReason,
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
            s.emit("message-back", format!("{:?}", state_for_debug.clone()))
                .ok();
        });

        let state_for_lobby = state.clone();
        let io_for_lobby_controller = io_inside.clone();
        s.on(
            "lobbyControl",
            |socket: SocketRef, Data::<LobbyPayload>(data)| {
                lobby_controller(socket, io_for_lobby_controller, data, state_for_lobby);
            },
        );

        let state_for_start_game = state.clone();
        let io_for_start_game = io_inside.clone();
        s.on("startGame", |socket: SocketRef, Data::<StartGamePayload>(data)| {
            start_game_controller(socket, data, state_for_start_game, io_for_start_game);
        });

        // let state_for_game_7 = state.clone();
        // let io_for_game_7 = io_inside.clone();
        // s.on("7Move", |socket: SocketRef, Data::<Game7Payload>(data)| {
        //     println!("{:?}",game_7_controller_with_error_handler(socket, data, state_for_game_7, io_for_game_7));
        // });

        let state_for_disconnect = state.clone();
        let io_for_disconnect = io_inside.clone();
        s.on_disconnect(|socket: SocketRef, reason: DisconnectReason| {
            disconnect_controller(socket, reason, state_for_disconnect, io_for_disconnect);
        });
    });
}
