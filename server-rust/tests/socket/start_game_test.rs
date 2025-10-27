use crate::socket::mocks::*;
use rust_socketio::{ClientBuilder, Payload, client::Client};
use server_rust::{
    objects::{Game7Logic, GameLogic, PlayerGameData, states::ServerState},
    responses::{LobbyResponse, seven_response::SevenGameStartResponse},
    run_test_server,
    socket::{
        ErrorResponse, LobbyPayload, lobby_socket::LobbyEvents, start_game_socket::StartGameEvents,
    },
};
use socketioxide::socket::DisconnectReason;
use std::sync::{
    Arc, Mutex,
    mpsc::{self, Receiver},
};

#[test]
fn test_create_7_lobby_200() {
    let (state, _rt, socket1, rx1) = setup_test_with_listener("startedGame");
    let (socket2, rx2) = connect_with_listener("startedGame");

    // --- Create lobby ---
    let data = LobbyPayload {
        username: Some("player1".to_string()),
        event_type: LobbyEvents::CreateLobby,
        lobby_id: None,
    };
    socket1
        .emit("lobbyControl", serde_json::to_value(&data).unwrap())
        .expect("emit failed");

    // --- Join lobby ---
    let data = LobbyPayload {
        username: Some("player2".to_string()),
        event_type: LobbyEvents::JoinLobby,
        lobby_id: Some(res_lobby_create.id),
    };
    socket2
        .emit("lobbyControl", serde_json::to_value(&data).unwrap())
        .expect("emit failed");

    // --- Start Game ---
    let data = StartGameEvents::Seven;
    socket1
        .emit("startGame", serde_json::to_value(&data).unwrap())
        .expect("emit failed");

    let msg1 = rx1
        .recv_timeout(std::time::Duration::from_secs(3))
        .expect("no response");
    let res1: SevenGameStartResponse =
        serde_json::from_str(&msg1).expect("invalid JSON in response");

    let msg2 = rx1
        .recv_timeout(std::time::Duration::from_secs(3))
        .expect("no response");
    let res2: SevenGameStartResponse =
        serde_json::from_str(&msg2).expect("invalid JSON in response");

    assert_eq!(res1.game_mode, "GameMode: 7");
    assert_eq!(res1.players_info[0].name, "player1");
    assert_eq!(res1.players_info[1].name, "player2");
    assert_eq!(res1.players_info[0].cards_left, 26);
    assert_eq!(res1.players_info[1].cards_left, 26);
    assert_eq!(res2.game_mode, "GameMode: 7");
    assert_eq!(res2.players_info[0].name, "player1");
    assert_eq!(res2.players_info[1].name, "player2");
    assert_eq!(res2.players_info[0].cards_left, 26);
    assert_eq!(res2.players_info[1].cards_left, 26);
    assert_eq!(res1.board, vec![vec![0; 4]; 3]);
    assert_eq!(
        [res1.hand_info, res2.hand_info]
            .concat()
            .into_iter()
            .sorted()
            .collect::<Vec<_>>(),
        (0..52).collect::<Vec<_>>()
    );

    let state = state.lock().unwrap();
    assert_eq!(state.player_lobby_map.len(), 2);
    assert!(state.game_map.contains_key(&res_lobby_create.id));

    let lobby_arc = state
        .game_map
        .get(&state.player_lobby_map.get(res1.players_info[0].id))
        .unwrap();
    let lobby = lobby_arc.lock().unwrap();
    let GameLogic::Game7Logic(ref game7) = *lobby else {
        panic!("Expected Game7Logic variant");
    };

    assert_eq!(game7.board, vec![vec![0; 4]; 3]);
    assert_eq!(game7.starting_player_id, game7.turn_manager.get_current());
    assert_eq!(game7.r#box, None);
    assert_eq!(game7.game_data.game_started, true);
    assert_eq!(game7.game_data.players.get_all().len(), 2);
    let mut deck: Vec<u32> = game7
        .game_data
        .players
        .get_all()
        .flat_map(|p| {
            if let PlayerGameData::Player7Data(d) = p.game.as_ref().unwrap() {
                d.hand.clone()
            } else {
                panic!("Should not have gotten to here");
            }
        })
        .collect();
    deck.sort();
    assert_eq!(deck, (0..52).collect::<Vec<_>>());
    let starting_player = "".to_string();
    for player in game7.game_data.players.get_all() {
        let PlayerGameData::Player7(ref player7) = *player else {
            panic!("Expected Game7Logic variant");
        };
        assert_eq!(player7.cards_left, 26);
        assert_eq!(player7.total_score, 0);
        if player7.hand.contains(19) {
            starting_player = player.get_id();
        }
    }
    assert_eq!(game7.starting_player_id, starting_player);
    assert_eq!(game7.turn_manager.get_current(), starting_player);

    assert!(
    game7.game_data.players.get_all().any(|p| p.get_id() == game7.turn_manager.get_next()),
    "Next player should exist in player list"
);
}
