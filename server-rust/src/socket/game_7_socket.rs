use colored::Colorize;
use serde::{Deserialize, Serialize};
use socketioxide::{SocketIo, extract::SocketRef};

use crate::{
    models::{GameLogic, Lobby, PlayerGameData},
    state::SharedState,
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Game7Payload {
    pub card: Option<i32>,
    pub move_type: Game7Events,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Game7Events {
    PlayAgain,
    SkipTurn,
    PlayCard,
}

pub fn game_7_controller(
    s: SocketRef,
    data: Game7Payload,
    state: SharedState,
    io: SocketIo,
) -> Result<String, String> {
    println!(
        "Game 7 Event:{} {}",
        format!("{:?}", data).blue(),
        s.id.to_string().green()
    );

    let state = state.lock().map_err(|_| "Failed to lock global state")?;

    let lobby_id = state
        .player_lobby
        .get(&s.id.to_string())
        .ok_or("Player not found in player_lobby")?;

    let lobby_arc = state
        .lobbies
        .get(lobby_id)
        .ok_or("Lobby not found for player")?;

    let mut lobby = lobby_arc.lock().map_err(|_| "Failed to lock lobby")?;

    // Verify it’s the player’s turn
    if let Some(GameLogic::Game7Logic(game)) = &lobby.game {
        let not_players_turn = s.id.to_string().as_str() != game.turn_manager.get_current()
            && data.move_type != Game7Events::PlayAgain;

        if not_players_turn {
            return Err("It is not your turn".to_string());
        }
    }

    // Handle the move type
    match data.move_type {
        Game7Events::PlayAgain => {
            // TODO: Implement replay logic
        }
        Game7Events::SkipTurn => {
            // TODO: Skip turn logic
        }
        Game7Events::PlayCard => {
            let card = data.card.ok_or("Missing card in payload")?;
            play_card_helper(&card, &mut lobby, &s)?;
        }
    }

    // TODO: LAV SÅ MAN SKIFTER TUR

    if let Some(GameLogic::Game7Logic(game)) = &lobby.game {
        game.response_move(&lobby, &io);
        game.response_hand(&lobby, &s);
    }

    Ok("hej".to_string())
}

fn play_card_helper(card: &i32, lobby: &mut Lobby, s: &SocketRef) -> Result<(), String> {
    let Some(GameLogic::Game7Logic(game)) = &lobby.game else {
        return Err("Lobby does not contain a Game7Logic instance".to_string());
    };

    if !card_playable(&card, &game.board) {
        return Err(format!("Card {} cannot be played", card));
    }

    let player = lobby
        .players
        .get_mut(&s.id.to_string())
        .ok_or("Player not found in lobby")?;

    if let Some(PlayerGameData::Player7(player_7_data)) = &mut player.game {
        if player_7_data.cards_left > 0 {
            let card2 = card.clone() as u32;
            player_7_data.hand = player_7_data
                .hand
                .iter()
                .filter(|&&hand_card| hand_card != card2)
                .cloned()
                .collect::<Vec<u32>>();
            player_7_data.cards_left = player_7_data.hand.len();
        } else {
            return Err("Player has no cards left".to_string());
        }
    } else {
        return Err("Player is not part of a Game7 session".to_string());
    }

    play_card(card, lobby)?;
    Ok(())
}

fn play_card(card: &i32, lobby: &mut Lobby) -> Result<(), String> {
    let game = match &mut lobby.game {
        Some(GameLogic::Game7Logic(game)) => game,
        _ => return Err("Lobby does not contain a Game7Logic instance".to_string()),
    };

    let suit = ((card - (card % 13)) / 13) as usize;
    let rank = card % 13 + 1;

    let row = if rank == 7 {
        1
    } else if rank < 7 {
        2
    } else {
        0
    };
    game.board[row][suit] = rank;

    Ok(())
}

pub fn card_playable(card: &i32, board: &Vec<Vec<i32>>) -> bool {
    let suit = ((card - (card % 13)) / 13) as usize;
    let rank = card % 13 + 1;
    // println!("{}, {}, {}",card, suit,rank);

    if board[1][1] == 0 && *card != 19 {
        return false;
    }
    if rank == 7 {
        return true;
    }
    if board[1][suit] == 0 {
        return false;
    }
    if rank == 6 || rank == 8 {
        return true;
    }
    if rank > 7 && board[0][suit] + 1 == rank {
        return true;
    }
    if rank < 7 && board[2][suit] - 1 == rank {
        return true;
    }
    false
}
