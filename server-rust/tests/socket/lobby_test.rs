use std::sync::{mpsc::{self, Receiver}, Arc, Mutex};
use crate::socket::mocks::*;



use rust_socketio::{client::Client, ClientBuilder, Payload};
use server_rust::{objects::{states::ServerState, Game7Logic, GameLogic}, responses::LobbyResponse, run_test_server, socket::{lobby_socket::LobbyEvents, ErrorResponse, LobbyPayload}};

#[test]
fn test_message_event() {
    let (_state, _rt, socket, rx) = setup_test_with_listener("message-back");
    socket.emit("message", "").expect("emit failed");

    let msg = rx
        .recv_timeout(std::time::Duration::from_secs(3))
        .expect("no response");
    assert_eq!(msg, "\"Hello World!\"");
}

#[test]
fn test_lobby_control_event_createlobby_no_name_400() {
    let (_state, _rt, socket, rx) = setup_test_with_listener("errorMessage");

    let data = LobbyPayload {
        username: None, /*Some("player1".to_string())*/
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    socket
        .emit("lobbyControl", serde_json::to_value(&data).unwrap())
        .expect("emit failed");

    let msg = rx
        .recv_timeout(std::time::Duration::from_secs(3))
        .expect("no response");
    // parse into your ErrorResponse struct
    let err: ErrorResponse = serde_json::from_str(&msg).expect("invalid JSON in response");

    assert_eq!(err.message, "You need to send a username");
    assert_eq!(err.r#type, "Lobby Error");
}

#[test]
fn test_lobby_control_event_createlobby_shortname_400() {
    let (_state, _rt, socket, rx) = setup_test_with_listener("errorMessage");


    let data = LobbyPayload {
        username: Some("a".to_string()),
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    socket
        .emit("lobbyControl", serde_json::to_value(&data).unwrap())
        .expect("emit failed");

    let msg = rx
        .recv_timeout(std::time::Duration::from_secs(3))
        .expect("no response");
    // parse into your ErrorResponse struct
    let err: ErrorResponse = serde_json::from_str(&msg).expect("invalid JSON in response");

    assert_eq!(err.message, "Your username did not uphold the high standards required");
    assert_eq!(err.r#type, "Lobby Error");
}

#[test]
fn test_lobby_control_event_createlobby_200() {
    let (state, _rt, socket, rx) = setup_test_with_listener("conToLobby");

    let data = LobbyPayload {
        username: Some("player1".to_string()),
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    socket
        .emit("lobbyControl", serde_json::to_value(&data).unwrap())
        .expect("emit failed");

    let msg = rx
        .recv_timeout(std::time::Duration::from_secs(3))
        .expect("no response");
    // parse into your ErrorResponse struct
    let res: LobbyResponse = serde_json::from_str(&msg).expect("invalid JSON in response");

    assert!((0..10).contains(&res.id));
    assert_eq!(res.players.len(), 1);
    assert_eq!(res.players[0].name, "player1");

    let state = state.lock().unwrap();
    assert_eq!(state.player_lobby_map.len(), 1);
    assert!(state.game_map.contains_key(&res.id));
    // let Ok(GameLogic::LobbyLogic(idk)): Option<_> = state.game_map.get(&res.id).unwrap().lock();

    let lobby_arc = state.game_map.get(&res.id).unwrap();
    let lobby = lobby_arc.lock().unwrap();
    let GameLogic::LobbyLogic(ref lobby) = *lobby;

    assert_eq!(lobby.get_players().get_all()[0].name, "player1");
    assert_eq!(lobby.get_game_id(), res.id);
}

#[test]
fn test_lobby_control_event_createlobby_and_join_200() {
    let (state, _rt, socket1, rx1) = setup_test_with_listener("conToLobby");
    let (socket2,rx2) = connect_with_listener("conToLobby");

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
    // parse into your ErrorResponse struct
    let res_lobby_create: LobbyResponse = serde_json::from_str(&msg).expect("invalid JSON in response");

    let data = LobbyPayload {
        username: Some("player2".to_string()),
        event_type: LobbyEvents::JoinLobby,
        lobby_id: Some(res_lobby_create.id),
    };
    socket2
        .emit("lobbyControl", serde_json::to_value(&data).unwrap())
        .expect("emit failed");

    let msg = rx2
        .recv_timeout(std::time::Duration::from_secs(3))
        .expect("no response");
    // parse into your ErrorResponse struct
    let res_join_lobby: LobbyResponse = serde_json::from_str(&msg).expect("invalid JSON in response");

    assert!((0..10).contains(&res_lobby_create.id));
    assert_eq!(res_lobby_create.players.len(), 1);
    assert_eq!(res_lobby_create.players[0].name, "player1");

    assert!(&res_lobby_create.id == &res_join_lobby.id);
    assert_eq!(res_join_lobby.players.len(), 2);
    assert_eq!(res_join_lobby.players[0].name, "player1");
    assert_eq!(res_join_lobby.players[1].name, "player2");

    let state = state.lock().unwrap();
    assert_eq!(state.player_lobby_map.len(), 2);
    assert!(state.game_map.contains_key(&res_lobby_create.id));
    // let Ok(GameLogic::LobbyLogic(idk)): Option<_> = state.game_map.get(&res.id).unwrap().lock();

    let lobby_arc = state.game_map.get(&res_lobby_create.id).unwrap();
    let lobby = lobby_arc.lock().unwrap();
    let GameLogic::LobbyLogic(ref lobby) = *lobby;

    assert_eq!(lobby.get_players().get_all()[0].name, "player1");
    assert_eq!(lobby.get_players().get_all()[1].name, "player2");
    assert_eq!(lobby.get_game_id(), res_lobby_create.id);
}

#[test]
fn test_lobby_control_event_createlobby_and_join_400() {
    let (state, _rt, socket1, rx1) = setup_test_with_listener("conToLobby");
    let (socket2,rx2) = connect_with_listener("errorMessage");

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
    // parse into your ErrorResponse struct
    let res_lobby_create: LobbyResponse = serde_json::from_str(&msg).expect("invalid JSON in response");

    let data = LobbyPayload {
        username: None,
        event_type: LobbyEvents::JoinLobby,
        lobby_id: Some(res_lobby_create.id),
    };
    socket2
        .emit("lobbyControl", serde_json::to_value(&data).unwrap())
        .expect("emit failed");

    let msg = rx2
        .recv_timeout(std::time::Duration::from_secs(3))
        .expect("no response");
    // parse into your ErrorResponse struct
    let err: ErrorResponse = serde_json::from_str(&msg).expect("invalid JSON in response");

    assert!((0..10).contains(&res_lobby_create.id));
    assert_eq!(res_lobby_create.players.len(), 1);
    assert_eq!(res_lobby_create.players[0].name, "player1");

    assert_eq!(err.message, "You need to send a username");
    assert_eq!(err.r#type, "Lobby Error");

    let state_gaurd = state.lock().unwrap();
    assert_eq!(state_gaurd.player_lobby_map.len(), 1);
    assert!(state_gaurd.game_map.contains_key(&res_lobby_create.id));
    // let Ok(GameLogic::LobbyLogic(idk)): Option<_> = state.game_map.get(&res.id).unwrap().lock();

    let lobby_arc = state_gaurd.game_map.get(&res_lobby_create.id).unwrap();
    let lobby = lobby_arc.lock().unwrap();
    let GameLogic::LobbyLogic(ref lobby) = *lobby;

    assert_eq!(lobby.get_players().get_all()[0].name, "player1");
    assert_eq!(lobby.get_game_id(), res_lobby_create.id);
}

