use colored::Colorize;
use serde::{Deserialize, Serialize};
use socketioxide::{SocketIo, extract::SocketRef};

use crate::{
    models::{GameLogic, Lobby, PlayerGameData},
    socket::{ErrorResponse, send_error_socket::send_error_message},
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

pub fn game_7_controller_with_error_handler(
    s: SocketRef,
    data: Game7Payload,
    state: SharedState,
    io: SocketIo,
) {
    println!(
        "Game 7 Event:{} {}",
        format!("{:?}", data).blue(),
        s.id.to_string().green()
    );

    match game_7_controller(s.id.to_string(), data, &state) {
        Ok(ResponseAmount::Single) => {
            let state = state.lock().unwrap();

            let lobby_id = state.player_lobby.get(&s.id.to_string()).unwrap();

            let lobby_arc = state.lobbies.get(lobby_id).unwrap();

            let mut lobby = lobby_arc.lock().unwrap();
            if let Some(GameLogic::Game7Logic(game)) = &lobby.game {
                game.response_move(&lobby, &io);
                game.response_hand(&lobby, &s);
            }
        }
        Ok(ResponseAmount::All) => todo!(),
        Err(err) => send_error_message(
            &s,
            ErrorResponse {
                message: err,
                r#type: "7 Game".to_string(),
            },
        ),
    }
}

pub enum ResponseAmount {
    Single,
    All,
}

pub fn game_7_controller(
    s_id: String,
    data: Game7Payload,
    state: &SharedState,
) -> Result<ResponseAmount, String> {
    let state = state.lock().map_err(|_| "Failed to lock global state")?;

    let lobby_id = state
        .player_lobby
        .get(&s_id)
        .ok_or("Player not found in player_lobby")?;

    let lobby_arc = state
        .lobbies
        .get(lobby_id)
        .ok_or("Lobby not found for player")?;

    let mut lobby = lobby_arc.lock().map_err(|_| "Failed to lock lobby")?;

    // Verify it’s the player’s turn
    if let Some(GameLogic::Game7Logic(game)) = &lobby.game {
        let not_players_turn = s_id.as_str() != game.turn_manager.get_current()
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
            let PlayerGameData::Player7(player_7_data) =
                lobby.players.get(&s_id).ok_or("Could not find player")?.game.as_ref().ok_or("Player has no active game")?
            else {
                return Err("Player is not in a Game7 match".to_string());
            };
            let Some(GameLogic::Game7Logic(game)) = &lobby.game else {
                return Err("Lobby does not contain a Game7Logic instance".to_string());
            };
            if possibleSkip(&player_7_data.hand, &game.board) {
                return Err("You can not skip!!!".to_string());
            }
            let Some(GameLogic::Game7Logic(game)) = &mut lobby.game else {
                return Err("Lobby does not contain a Game7Logic instance".to_string());
            };
            game.set_box(s_id.to_string());
        }
        Game7Events::PlayCard => {
            let card = data
                .card
                .ok_or("Missing card in payload")
                .and_then(|card| {
                    if (0..=51).contains(&card) {
                        Ok(card)
                    } else {
                        Err("Card is out of range")
                    }
                })?;
            play_card_helper(&card, &mut lobby, &s_id)?;
        }
    }

    let players_ref = &lobby.players.clone();
    if let Some(GameLogic::Game7Logic(game)) = &mut lobby.game {
        game.turn_manager
            .advance_turn(players_ref)
            .map_err(|_| "Could not adavnce turn")?;
    }

    Ok(ResponseAmount::Single)
}

fn play_card_helper(card: &i32, lobby: &mut Lobby, s_id: &String) -> Result<(), String> {
    let Some(GameLogic::Game7Logic(game)) = &lobby.game else {
        return Err("Lobby does not contain a Game7Logic instance".to_string());
    };

    if !card_playable(&card, &game.board) {
        return Err(format!("Card {} cannot be played", card));
    }

    let player = lobby
        .players
        .get_mut(&s_id)
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

pub fn possibleSkip(hand: &Vec<u32>, board: &Vec<Vec<i32>>) -> bool {
    hand.iter()
        .any(|card| card_playable(&(*card as i32), board))
}
