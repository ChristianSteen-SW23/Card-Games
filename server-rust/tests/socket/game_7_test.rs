use crate::socket::{mocks::*, mocks_7::create_7game};
use rust_socketio::{ClientBuilder, Payload, client::Client};

use server_rust::{
    objects::GameLogic,
    responses::seven_response::{
        SevenGameEndedResponse, SevenGameStartResponse, SevenGameUpdateResponse,
        SevenHandUpdateResponse,
    },
    socket::{Game7Payload, game_7_socket::Game7Events::PlayCard},
};
use socketioxide::socket::DisconnectReason;
use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver},
    },
    time::Duration,
};

#[test]
fn test_play_card_200() {
    let (state, rt, socket1, rx1, socket2, rx2, lobby_id) = create_7game();
    let _: SevenGameStartResponse = recv(&rx1["startedGame"]);

    // Set up hands/turn, then release the locks before triggering the move.
    let (player1_id, board_snapshot) = {
        let state_guard = state.lock().unwrap();
        let lobby_arc = state_guard.game_map.get(&lobby_id).unwrap();
        let mut lobby = lobby_arc.lock().unwrap();
        let GameLogic::Game7(ref mut game7) = *lobby else {
            panic!("Expected Game7Logic variant");
        };

        game7.players[0].hand = (0..=25).collect();
        game7.players[1].hand = (25..=51).collect();
        let _ = game7.turn_manager.update(
            game7.players[0].id.clone(),
            &game7.players.iter().map(|p| p.id.clone()).collect(),
        );

        (game7.players[0].id.clone(), game7.board.clone())
        // `lobby` and `state_guard` drop here, at the end of this block
    };

    let action = Game7Payload {
        card: Some(19),
        move_type: PlayCard,
    };
    let res1: SevenGameUpdateResponse = emit_and_recv(&socket1, &rx1["gameInfo"], "7Move", &action);

    let hand1: SevenHandUpdateResponse = recv(&rx1["handInfo"]);
    let res2: SevenGameUpdateResponse = recv(&rx2["gameInfo"]);

    // Re-lock only for the final assertions, after everything's settled.
    let state_guard = state.lock().unwrap();
    let lobby_arc = state_guard.game_map.get(&lobby_id).unwrap();
    let lobby = lobby_arc.lock().unwrap();
    let GameLogic::Game7(ref game7) = *lobby else {
        panic!("Expected Game7Logic variant");
    };

    assert_eq!(game7.board, [[0, 0, 0, 0], [0, 7, 0, 0], [0, 0, 0, 0]]);
    assert_eq!(game7.players.len(), 2);

    assert_eq!(res1, res2);
    assert_eq!(
        hand1.hand_info,
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22, 23, 24,
            25
        ]
    )
}



#[test]
fn test_play_card_with_win_200() {
    let (state, rt, socket1, rx1, socket2, rx2, lobby_id) = create_7game();
    let _: SevenGameStartResponse = recv(&rx1["startedGame"]);

    // Set up hands/turn, then release the locks before triggering the move.
    let (player1_id, board_snapshot) = {
        let state_guard = state.lock().unwrap();
        let lobby_arc = state_guard.game_map.get(&lobby_id).unwrap();
        let mut lobby = lobby_arc.lock().unwrap();
        let GameLogic::Game7(ref mut game7) = *lobby else {
            panic!("Expected Game7Logic variant");
        };

        game7.players[0].hand = (19..=19).collect();
        game7.players[0].cards_left = 1;
        game7.players[1].hand = (0..=12).collect();
        let _ = game7.turn_manager.update(
            game7.players[0].id.clone(),
            &game7.players.iter().map(|p| p.id.clone()).collect(),
        );

        (game7.players[0].id.clone(), game7.board.clone())
        // `lobby` and `state_guard` drop here, at the end of this block
    };

    let action = Game7Payload {
        card: Some(19),
        move_type: PlayCard,
    };
    let res1: SevenGameEndedResponse = emit_and_recv(&socket1, &rx1["gameEnded"], "7Move", &action);
    let res2: SevenGameEndedResponse = recv(&rx2["gameEnded"]);

    // Re-lock only for the final assertions, after everything's settled.
    let state_guard = state.lock().unwrap();
    let lobby_arc = state_guard.game_map.get(&lobby_id).unwrap();
    let lobby = lobby_arc.lock().unwrap();
    let GameLogic::Game7(ref game7) = *lobby else {
        panic!("Expected Game7Logic variant");
    };
    println!("{:?}",game7);
        assert_eq!(game7.board, [[0, 0, 0, 0], [0, 7, 0, 0], [0, 0, 0, 0]]);
    assert_eq!(game7.players.len(), 2);
    assert_eq!(game7.players[0].cur_score, 0);
    assert_eq!(game7.players[0].total_score, 0);
    assert_eq!(game7.players[1].cur_score, 90);
    assert_eq!(game7.players[1].total_score, 90);

    assert_eq!(res1, res2);

    assert_eq!(res1.win_data.len(), 2);
    assert_eq!(res1.win_data[0].name, "player1");
    assert_eq!(res1.win_data[0].round_score, 0);
    assert_eq!(res1.win_data[0].total_score, 0);
    assert_eq!(res1.win_data[1].name, "player2");
    assert_eq!(res1.win_data[1].round_score, 90);
    assert_eq!(res1.win_data[1].total_score, 90);
}