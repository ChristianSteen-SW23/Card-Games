use std::sync::{mpsc::{self, Receiver}, Arc, Mutex};
use crate::socket::mocks::*;
use rust_socketio::{client::Client, ClientBuilder, Payload};
use server_rust::{objects::{states::ServerState, Game7Logic, GameLogic}, responses::LobbyResponse, run_test_server, socket::{lobby_socket::LobbyEvents, ErrorResponse, LobbyPayload}};
use socketioxide::socket::DisconnectReason;

// #[test]
fn test_disconnect_from_lobby() {
    let (state, _rt, socket1, rx1) = setup_test_with_listener("conToLobby");
    let (socket2, rx2) = connect_with_listener("leaveLobby");

    // --- Create lobby ---
    let data = LobbyPayload {
        username: Some("player1".to_string()),
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    socket1
        .emit("lobbyControl", serde_json::to_value(&data).unwrap())
        .expect("emit failed");

    let msg = rx1
        .recv_timeout(std::time::Duration::from_secs(3))
        .expect("no response");
    let res_lobby_create: LobbyResponse =
        serde_json::from_str(&msg).expect("invalid JSON in response");

    // --- Join lobby ---
    let data = LobbyPayload {
        username: Some("player2".to_string()),
        event_type: LobbyEvents::JoinLobby,
        lobby_id: Some(res_lobby_create.id),
    };
    socket2
        .emit("lobbyControl", serde_json::to_value(&data).unwrap())
        .expect("emit failed");

    // --- Disconnect player1 ---
    socket1.disconnect().expect("failed to disconnect");

    let _msg = rx2
        .recv_timeout(std::time::Duration::from_secs(3))
        .expect("no response");

    assert!((0..10).contains(&res_lobby_create.id));
    assert_eq!(res_lobby_create.players.len(), 1);
    assert_eq!(res_lobby_create.players[0].name, "player1");

    let state = state.lock().unwrap();
    assert_eq!(state.player_lobby_map.len(), 0);
    assert!(!state.game_map.contains_key(&res_lobby_create.id));
}