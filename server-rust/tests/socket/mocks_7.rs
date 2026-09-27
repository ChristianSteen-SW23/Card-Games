use rust_socketio::client::Client;
use server_rust::{
    objects::states::ServerState,
    responses::LobbyResponse,
    socket::{
        LobbyPayload,
        lobby_socket::LobbyEvents,
        start_game_socket::{StartGameEvents::Seven, StartGamePayload},
    },
};
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        mpsc::Receiver,
    },
};
use tokio::runtime::Runtime;

use crate::socket::mocks::{connect_with_listeners, emit_and_recv, setup_test_with_listeners};

pub fn create_7game() -> (
    Arc<Mutex<ServerState>>,
    Runtime,
    Client,
    HashMap<String, Receiver<String>>,
    Client,
    HashMap<String, Receiver<String>>,
    u32
) {
    let (state, rt, socket1, rx1) = setup_test_with_listeners(&[
        "conToLobby",
        "playerHandler",
        "startedGame",
        "gameInfo",
        "handInfo",
        "gameEnded",
    ]);
    let (socket2, rx2) = connect_with_listeners(&[
        "conToLobby",
        "playerHandler",
        "startedGame",
        "gameInfo",
        "handInfo",
        "gameEnded",
    ]);

    let create_data = LobbyPayload {
        username: Some("player1".to_string()),
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    let res_lobby_create: LobbyResponse =
        emit_and_recv(&socket1, &rx1["conToLobby"], "lobbyControl", &create_data);

    let join_data = LobbyPayload {
        username: Some("player2".to_string()),
        event_type: LobbyEvents::JoinLobby,
        lobby_id: Some(res_lobby_create.id),
    };
    let _: LobbyResponse = emit_and_recv(&socket2, &rx2["conToLobby"], "lobbyControl", &join_data);

    let start_game = StartGamePayload { game_mode: Seven };
    socket1
        .emit("startGame", serde_json::to_value(start_game).unwrap())
        .expect("emit failed");

    (state, rt, socket1, rx1, socket2, rx2, res_lobby_create.id)
}
