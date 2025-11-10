use colored::Colorize;
use serde::{Deserialize, Serialize};
use socketioxide::{SocketIo, extract::SocketRef};

use crate::{
    objects::{Game7Logic, GameLogic, states::SharedState},
    responses::{Event, Planned, Responses, seven_response::SevenGameStartResponse},
    socket::send_error_socket::Error,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartGamePayload {
    pub game_mode: StartGameEvents,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum StartGameEvents {
    #[serde(rename = "7")]
    Seven,
    #[serde(rename = "31")]
    ThirtyOne,
    #[serde(rename = "500")]
    FiveHundred,
    #[serde(rename = "Planning Poker")]
    PlanningPoker,
}

// pub fn start_game_controller(
//     s: SocketRef,
//     playload_data: StartGamePayload,
//     state: SharedState,
//     io: SocketIo,
// ) {
//     println!(
//         "Lobby Event:{} {}",
//         format!("{:?}", playload_data).blue(),
//         s.id.to_string().green()
//     );

//     let mut state = state.lock().unwrap();

//     let result: Result<Responses, Error> = {
//         let lobby_arc = state.get_lobby_arc(s.id.as_str())?;
//         let mut lobby_guard = lobby_arc.lock().unwrap();

//         match *lobby_guard {
//             GameLogic::LobbyLogic(lobby_logic) => {
//                 let new_game = GameLogic::Game7Logic(Game7Logic::new_from_lobby(lobby_logic));
//                 *lobby_guard = new_game;
//                 todo!()
//             }
//             _ => Err(Error::LobbyError("Expected LobbyLogic".into())),
//         }
//     };

//     match result {
//         Err(e) => e.emit_error_response(&s),
//         Ok(value) => value.emit_ok_response(&s, &io, lobby_id),
//     }
// }

pub fn start_game_controller(
    s: SocketRef,
    payload: StartGamePayload,
    state: SharedState,
    io: SocketIo,
) {
    use colored::Colorize;

    println!(
        "Lobby Event:{} {}",
        format!("{:?}", payload).blue(),
        s.id.to_string().green()
    );

    // we cannot use `?` here, so handle errors explicitly and return
    let state_guard = state.lock().unwrap();

    // find lobby_id from player map
    let lobby_id = match state_guard.player_lobby_map.get(&s.id.to_string()).cloned() {
        Some(id) => id,
        None => {
            Error::LobbyError("Player not in any lobby".into()).emit_error_response(&s);
            return;
        }
    };

    // get the Arc<Mutex<GameLogic>>
    let lobby_arc = match state_guard.game_map.get(&lobby_id).cloned() {
        Some(arc) => arc,
        None => {
            Error::LobbyError("Lobby not found".into()).emit_error_response(&s);
            return;
        }
    };

    // mutate the game in-place
    let mut game_guard = lobby_arc.lock().unwrap();

    let result: Result<Responses, Error> = match &*game_guard {
        GameLogic::LobbyLogic(lobby_logic) => {
            match payload.game_mode {
                StartGameEvents::Seven => {
                    // Build the new game
                    let new_game = Game7Logic::new_from_lobby(lobby_logic.to_owned());

                    // Move it into the guard
                    *game_guard = GameLogic::Game7Logic(new_game);

                    // Borrow it back immutably
                    let GameLogic::Game7Logic(ref game7_ref) = *game_guard else {
                        Error::LobbyError("Failed to assign new Game7Logic".into())
                            .emit_error_response(&s);
                        return;
                    };

                    // Build responses for all players
                    let responses = game7_ref
                        .game_data
                        .players
                        .get_all()
                        .iter()
                        .map(|player| {
                            Planned::new(
                                Event::StartedGame,
                                crate::responses::EmitContext::SingleString {
                                    io: &io,
                                    sid: player.id.to_string(),
                                },
                                &SevenGameStartResponse::from((player.id.as_str(), game7_ref)),
                            )
                        })
                        .collect();
                    Ok(Responses::Multiple(responses))
                }
                StartGameEvents::ThirtyOne => todo!(),
                StartGameEvents::FiveHundred => todo!(),
                StartGameEvents::PlanningPoker => todo!(),
            }
        }
        _ => Err(Error::LobbyError(
            "Expected LobbyLogic before starting".into(),
        )),
    };

    match result {
        Err(e) => e.emit_error_response(&s),
        Ok(responses) => responses.emit_all(),
    }
}

// let mut state = state.lock().unwrap();

// let result: Result<Responses, Error> = {
//     let mut lobby = state.get_lobby_mut(s.id.as_str())?;
//     match playload_data.game_mode {
//         StartGameEvents::Seven => {
//             lobby = GameLogic::Game7Logic(Game7Logic::new_from_lobby(lobby));
//             todo!()
//         }
//         StartGameEvents::ThirtyOne => todo!(),
//         StartGameEvents::FiveHundred => todo!(),
//         StartGameEvents::PlanningPoker => todo!(),
//     }
// };
