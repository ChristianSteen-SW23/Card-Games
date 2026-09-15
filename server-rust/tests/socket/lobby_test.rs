use crate::socket::mocks::*;
use server_rust::{
    responses::LobbyResponse,
    socket::{ErrorResponse, LobbyPayload, lobby_socket::LobbyEvents},
};

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
    let data = LobbyPayload {
        username: None,
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };

    let err: ErrorResponse = socket_roundtrip("errorMessage", "lobbyControl", &data);

    assert_eq!(err.message, "You need to send a username");
    assert_eq!(err.r#type, "Lobby Error");
}

#[test]
fn test_lobby_control_event_createlobby_shortname_400() {
    let data = LobbyPayload {
        username: Some("a".to_string()),
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };

    let err: ErrorResponse = socket_roundtrip("errorMessage", "lobbyControl", &data);

    assert_eq!(
        err.message,
        "Your username did not uphold the high standards required"
    );
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
    let res: LobbyResponse = emit_and_recv(&socket, &rx, "lobbyControl", &data);

    assert!((0..10).contains(&res.id));
    assert_eq!(res.players.len(), 1);
    assert_eq!(res.players[0].name, "player1");

    assert_lobby_state(&state, res.id, &["player1"]);
}

#[test]
fn test_lobby_control_event_createlobby_and_join_200() {
    let (state, _rt, socket1, rx1) = setup_test_with_listeners(&["conToLobby", "playerHandler"]);
    let (socket2, rx2) = connect_with_listeners(&["conToLobby"]);

    let create_data = LobbyPayload {
        username: Some("player1".to_string()),
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    let res_lobby_create: LobbyResponse =
        emit_and_recv(&socket1, &rx1["conToLobby"], "lobbyControl", &create_data);

    assert!((0..10).contains(&res_lobby_create.id));
    assert_eq!(res_lobby_create.players.len(), 1);
    assert_eq!(res_lobby_create.players[0].name, "player1");

    let join_data = LobbyPayload {
        username: Some("player2".to_string()),
        event_type: LobbyEvents::JoinLobby,
        lobby_id: Some(res_lobby_create.id),
    };
    let res_join_lobby: LobbyResponse =
        emit_and_recv(&socket2, &rx2["conToLobby"], "lobbyControl", &join_data);

    assert_eq!(res_lobby_create.id, res_join_lobby.id);
    assert_eq!(res_join_lobby.players.len(), 2);
    assert_eq!(res_join_lobby.players[0].name, "player1");
    assert_eq!(res_join_lobby.players[1].name, "player2");

    // player1 gets the join broadcast on "playerHandler", not "conToLobby"
    let res_lobby_update: LobbyResponse = recv(&rx1["playerHandler"]);

    assert_eq!(res_lobby_update.id, res_lobby_create.id);
    assert_eq!(res_lobby_update.players.len(), 2);
    assert_eq!(res_lobby_update.players[0].name, "player1");
    assert_eq!(res_lobby_update.players[1].name, "player2");

    assert_lobby_state(&state, res_lobby_create.id, &["player1", "player2"]);
}

#[test]
fn test_lobby_control_event_createlobby_and_join_many_players_200() {
    let (state, _rt, socket1, rx1) = setup_test_with_listeners(&["conToLobby", "playerHandler"]);

    let create_data = LobbyPayload {
        username: Some("player1".to_string()),
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    let res_lobby_create: LobbyResponse =
        emit_and_recv(&socket1, &rx1["conToLobby"], "lobbyControl", &create_data);

    assert!((0..10).contains(&res_lobby_create.id));
    assert_eq!(res_lobby_create.players.len(), 1);
    assert_eq!(res_lobby_create.players[0].name, "player1");

    let mut res: LobbyResponse = LobbyResponse {
        id: 123,
        players: vec![],
    };
    for i in 2..10 {
        let (socket2, rx2) = connect_with_listeners(&["conToLobby"]);
        let join_data = LobbyPayload {
            username: Some(format!("player{i}").to_string()),
            event_type: LobbyEvents::JoinLobby,
            lobby_id: Some(res_lobby_create.id),
        };
        let _: LobbyResponse =
            emit_and_recv(&socket2, &rx2["conToLobby"], "lobbyControl", &join_data);
        res = recv(&rx1["playerHandler"]);
    }

    // player1 gets the join broadcast on "playerHandler", not "conToLobby"

    assert_eq!(res.id, res_lobby_create.id);
    assert_eq!(res.players.len(), 9);
    assert_eq!(res.players[0].name, "player1");
    assert_eq!(res.players[1].name, "player2");
    assert_eq!(res.players[2].name, "player3");
    assert_eq!(res.players[3].name, "player4");
    assert_eq!(res.players[4].name, "player5");
    assert_eq!(res.players[5].name, "player6");
    assert_eq!(res.players[6].name, "player7");
    assert_eq!(res.players[7].name, "player8");
    assert_eq!(res.players[8].name, "player9");
    // assert_eq!(res.players[9].name, "player10");

    assert_lobby_state(
        &state,
        res_lobby_create.id,
        &[
            "player1", "player2", "player3", "player4", "player5", "player6", "player7", "player8",
            "player9",
        ],
    );
}

#[test]
fn test_lobby_same_socket_not_join_the_two_lobbys() {
    let (state, _rt, socket1, rx1) = setup_test_with_listeners(&["conToLobby", "playerHandler"]);
    let (socket2, rx2) = connect_with_listeners(&["conToLobby", "errorMessage"]);

    let create_data = LobbyPayload {
        username: Some("player1".to_string()),
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    let res_lobby_create: LobbyResponse =
        emit_and_recv(&socket1, &rx1["conToLobby"], "lobbyControl", &create_data);

    let join_data = LobbyPayload {
        username: Some(format!("player2").to_string()),
        event_type: LobbyEvents::JoinLobby,
        lobby_id: Some(res_lobby_create.id),
    };
    let _: LobbyResponse = emit_and_recv(&socket2, &rx2["conToLobby"], "lobbyControl", &join_data);

    let join_data = LobbyPayload {
        username: Some(format!("player3").to_string()),
        event_type: LobbyEvents::JoinLobby,
        lobby_id: Some(res_lobby_create.id),
    };
    let err: ErrorResponse =
        emit_and_recv(&socket2, &rx2["errorMessage"], "lobbyControl", &join_data);

    assert_eq!(
        err.message,
        format!("Socket is already in a game")
    );
    assert_eq!(err.r#type, "Lobby Error");

    assert_lobby_state(&state, res_lobby_create.id, &["player1", "player2"]);
}
