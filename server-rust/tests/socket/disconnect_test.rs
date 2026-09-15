use core::time;
use std::{collections::HashMap, sync::mpsc::Receiver, thread::sleep};

use crate::socket::mocks::*;
use rust_socketio::client::Client;
use server_rust::{
    responses::LobbyResponse,
    socket::{LobbyPayload, lobby_socket::LobbyEvents},
};

#[test]
fn test_disconnect_from_lobby() {
    let (state, _rt, socket1, rx1) = setup_test_with_listeners(&["conToLobby", "playerHandler"]);
    let (socket2, rx2) = connect_with_listeners(&["conToLobby"]);

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
    let res_join_lobby: LobbyResponse =
        emit_and_recv(&socket2, &rx2["conToLobby"], "lobbyControl", &join_data);

    let res_lobby_update: LobbyResponse = recv(&rx1["playerHandler"]);

    socket2.disconnect().expect("failed to disconnect");
    let res_lobby_update: LobbyResponse = recv(&rx1["playerHandler"]);

    assert_eq!(res_lobby_update.id, res_lobby_create.id);
    assert_eq!(res_lobby_update.players.len(), 1);
    assert_eq!(res_lobby_update.players[0].name, "player1");

    assert_lobby_state(&state, res_lobby_create.id, &["player1"]);
}

#[test]
fn test_disconnect_from_big_lobby() {
    let (state, _rt, socket1, rx1) = setup_test_with_listeners(&["conToLobby", "playerHandler"]);

    let create_data = LobbyPayload {
        username: Some("player1".to_string()),
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    let res_lobby_create: LobbyResponse =
        emit_and_recv(&socket1, &rx1["conToLobby"], "lobbyControl", &create_data);

    let mut joined: Vec<(Client, HashMap<String, Receiver<String>>)> = Vec::new();

    for i in 2..10 {
        let (socket_n, rx_n) = connect_with_listeners(&["conToLobby", "playerHandler"]);
        let join_data = LobbyPayload {
            username: Some(format!("player{i}")),
            event_type: LobbyEvents::JoinLobby,
            lobby_id: Some(res_lobby_create.id),
        };
        let _: LobbyResponse = emit_and_recv(&socket_n, &rx_n["conToLobby"], "lobbyControl", &join_data);
        joined.push((socket_n, rx_n));
    }

    let (_, last_rx) = joined.last().unwrap();
    let _join_broadcast: LobbyResponse = recv(&last_rx["playerHandler"]);

    socket1.disconnect().expect("failed to disconnect");
    let res: LobbyResponse = recv(&last_rx["playerHandler"]);

    assert_eq!(res.id, res_lobby_create.id);
    assert_eq!(res.players.len(), 8);
    assert_eq!(res.players[0].name, "player2");
    assert_eq!(res.players[1].name, "player3");
    assert_eq!(res.players[2].name, "player4");
    assert_eq!(res.players[3].name, "player5");
    assert_eq!(res.players[4].name, "player6");
    assert_eq!(res.players[5].name, "player7");
    assert_eq!(res.players[6].name, "player8");
    assert_eq!(res.players[7].name, "player9");
    assert!(res.players.iter().any(|p| p.host));

    assert_lobby_state(
        &state,
        res_lobby_create.id,
        &[
            "player2", "player3", "player4", "player5", "player6", "player7", "player8", "player9",
        ],
    );
}


#[test]
fn test_disconnect_from_single_player_lobby_removes_lobby() {
    let (state, _rt, socket1, rx1) = setup_test_with_listeners(&["conToLobby"]);

    let create_data = LobbyPayload {
        username: Some("player1".to_string()),
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    let res_lobby_create: LobbyResponse =
        emit_and_recv(&socket1, &rx1["conToLobby"], "lobbyControl", &create_data);

    {
        let guard = state.lock().unwrap();
        assert!(guard.game_map.contains_key(&res_lobby_create.id));
        assert_eq!(guard.player_lobby_map.len(), 1);
    }

    socket1.disconnect().expect("failed to disconnect");

    // no broadcast to wait on here (room is empty) — poll until the server
    // has actually finished tearing the lobby down
    wait_until(&state, |s| !s.game_map.contains_key(&res_lobby_create.id));

    let guard = state.lock().unwrap();
    assert!(
        !guard.game_map.contains_key(&res_lobby_create.id),
        "lobby should be removed from game_map after the only player disconnects"
    );
    assert!(
        guard.player_lobby_map.is_empty(),
        "player_lobby_map should be cleaned up after the only player disconnects"
    );
}