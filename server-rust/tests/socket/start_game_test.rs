use crate::socket::{mocks::*, mocks_7::create_7game};
use rust_socketio::{ClientBuilder, Payload, client::Client};

use server_rust::{objects::GameLogic, responses::seven_response::SevenGameStartResponse};
use socketioxide::socket::DisconnectReason;
use std::{sync::{
    Arc, Mutex,
    mpsc::{self, Receiver},
}, time::Duration};

#[test]
fn test_create_7_lobby_200() {
    let (state, rt, socket1, rx1, socket2, rx2, lobby_id) = create_7game();

    let msg1: SevenGameStartResponse = recv(&rx1["startedGame"]);
    let msg2: SevenGameStartResponse = recv(&rx2["startedGame"]);

    assert_eq!(msg1.game_mode, "7");
    assert_eq!(msg1.players_info[0].name, "player1");
    assert_eq!(msg1.players_info[1].name, "player2");
    assert_eq!(msg1.players_info[0].cards_left, 26);
    assert_eq!(msg1.players_info[1].cards_left, 26);
    assert_eq!(msg2.game_mode, "7");
    assert_eq!(msg2.players_info[0].name, "player1");
    assert_eq!(msg2.players_info[1].name, "player2");
    assert_eq!(msg2.players_info[0].cards_left, 26);
    assert_eq!(msg2.players_info[1].cards_left, 26);
    assert_eq!(msg1.board, vec![vec![0; 4]; 3]);
    let mut all_cards: Vec<_> = msg1
        .hand_info
        .iter()
        .chain(&msg2.hand_info)
        .cloned()
        .collect();
    all_cards.sort();

    assert_eq!(all_cards, (0..52).collect::<Vec<_>>());

    let state = state.lock().unwrap();
    assert_eq!(state.player_lobby_map.len(), 2);

    let lobby_arc = state.game_map.get(&lobby_id).unwrap();
    let lobby = lobby_arc.lock().unwrap();
    let GameLogic::Game7(ref game7) = *lobby else {
        panic!("Expected Game7Logic variant");
    };

    assert_eq!(game7.board, vec![vec![0; 4]; 3]);
    assert_eq!(game7.starting_player_id, game7.turn_manager.get_current());
    assert_eq!(game7.r#box, None);
    assert_eq!(game7.players.len(), 2);
    let mut deck: Vec<u32> = game7.players
        .iter()
        .flat_map(|p| {
            p.hand.clone()
        })
        .collect();
    deck.sort();
    assert_eq!(deck, (0..52).collect::<Vec<_>>());
    let mut starting_player = "".to_string();
    for player in game7.players.clone() {
        assert_eq!(player.cards_left, 26);
        assert_eq!(player.total_score, 0);
        if player.hand.contains(&19) {
            starting_player = player.id.to_string();
        }
    }
    assert_eq!(game7.starting_player_id, starting_player);
    assert_eq!(game7.turn_manager.get_current(), starting_player);

    assert!(
        game7.players
            .iter()
            .any(|p| p.id == game7.turn_manager.get_next()),
        "Next player should exist in player list"
    );
}
