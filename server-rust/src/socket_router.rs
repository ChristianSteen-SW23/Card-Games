use crate::{
    objects::states::SharedState, socket::{
        Game7Payload, Game31Payload, disconnect_controller, game_7_socket::game_7_controller, game_31_socket::game_31_controller, lobby_socket::{LobbyPayload, lobby_controller}, start_game_controller, start_game_socket::StartGamePayload,
    },
};
use colored::Colorize;
use serde_json::Value;
use socketioxide::extract::Event;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef, TryData},
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

        // let state_for_debug = state.clone();
        // s.on("debug", move |s: SocketRef| {
        //     s.emit("message-back", format!("{:?}", state_for_debug.clone()))
        //         .ok();
        // });

        let state_for_lobby = state.clone();
        let io_for_lobby_controller = io_inside.clone();
        s.on(
            "lobbyControl",
            |socket: SocketRef, TryData::<LobbyPayload>(data)| match data {
                Ok(data) => {
                    lobby_controller(socket, io_for_lobby_controller, data, state_for_lobby)
                }
                Err(err) => log_bad_payload("lobbyControl", &socket, err),
            },
        );

        let state_for_start_game = state.clone();
        let io_for_start_game = io_inside.clone();
        s.on(
            "startGame",
            |socket: SocketRef, TryData::<StartGamePayload>(data)| match data {
                Ok(data) => {
                    start_game_controller(socket, data, state_for_start_game, io_for_start_game)
                }
                Err(err) => log_bad_payload("startGame", &socket, err),
            },
        );

        let state_for_game_7 = state.clone();
        let io_for_game_7 = io_inside.clone();
        s.on(
            "7Move",
            |socket: SocketRef, TryData::<Game7Payload>(data)| match data {
                Ok(data) => game_7_controller(socket, data, state_for_game_7, io_for_game_7),
                Err(err) => log_bad_payload("7Move", &socket, err),
            },
        );

        let state_for_game_31 = state.clone();
        let io_for_game_31 = io_inside.clone();
        s.on(
            "31Move",
            |socket: SocketRef, TryData::<Game31Payload>(data)| match data {
                Ok(data) => game_31_controller(socket, data, state_for_game_31, io_for_game_31),
                Err(err) => log_bad_payload("31Move", &socket, err),
            },
        );

        
        let state_for_disconnect = state.clone();
        let io_for_disconnect = io_inside.clone();
        s.on_disconnect(|socket: SocketRef, reason: DisconnectReason| {
            disconnect_controller(socket, reason, state_for_disconnect, io_for_disconnect);
        });

        s.on_fallback(|socket: SocketRef, Event(event), Data::<Value>(data)| {
            println!(
                "Unhandled event '{}' from {} with payload: {}",
                event.yellow(),
                socket.id.to_string().green(),
                data
            );
        });
    });
}

fn log_bad_payload(event: &str, socket: &SocketRef, err: impl std::fmt::Display) {
    println!(
        "Bad payload for '{}' from {}: {}",
        event.yellow(),
        socket.id.to_string().green(),
        err.to_string().red()
    );
}
