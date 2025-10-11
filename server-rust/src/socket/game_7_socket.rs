use colored::Colorize;
use serde::{Deserialize, Serialize};
use socketioxide::{SocketIo, extract::SocketRef};

use crate::{
    models::{GameLogic, Lobby, PlayerGameData, lobby, turn_manager},
    state::SharedState,
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Game7Payload {
    pub card: Option<u32>,
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
        let not_players_turn = Some(s.id.to_string()) != game.turn_manager.current
            && data.move_type != Game7Events::PlayAgain;

        if not_players_turn {
            println!("It's not your turn!");
        } else {
            println!("It's your turn!");
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

    if let Some(GameLogic::Game7Logic(game)) = &lobby.game {
        game.response_move(&lobby, &io);
        game.response_hand(&lobby, &s);
    }

    Ok("hej".to_string())
}

fn play_card_helper(card: &u32, lobby: &mut Lobby, s: &SocketRef) -> Result<(), String> {
    if card_playable(&card, &lobby) {
        return Err(format!("Card {} cannot be played", card));
    }

    let player = lobby
        .players
        .get_mut(&s.id.to_string())
        .ok_or("Player not found in lobby")?;

    if let Some(PlayerGameData::Player7(player_7_data)) = &mut player.game {
        if player_7_data.cards_left > 0 {
            player_7_data.cards_left -= 1;
        } else {
            return Err("Player has no cards left".to_string());
        }
    } else {
        return Err("Player is not part of a Game7 session".to_string());
    }

    play_card(card, lobby)?;
    
    Ok(())
}

fn play_card(card: &u32, lobby: &mut Lobby) ->Result<(), String> {
    let game = match &mut lobby.game {
        Some(GameLogic::Game7Logic(game)) => game,
        _ => return Err("Lobby does not contain a Game7Logic instance".to_string()),
    };


    let suit = ((card - (card % 13)) / 13) as usize;
    let rank = card % 13 + 1;

    let row = if rank == 7 { 1 } else if rank < 7 { 2 } else { 0 };
    game.board[row][suit] = rank;

    Ok(())
}

fn card_playable(card: &u32, lobby: &Lobby) -> bool {
    let Some(GameLogic::Game7Logic(game)) = &lobby.game else {
        return false;
    };

    let board = &game.board;

    let suit = ((card - (card % 13)) / 13) as usize;
    let rank = card % 13 + 1;

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
