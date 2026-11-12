use crate::objects::game31::Game31;
use crate::objects::lobby::settings_lobby::GameModeSettings;
use crate::objects::planning_poker::game_planning_poker::PlanningPoker;
use crate::responses::planning_poker_response::PlanningPokerResponse;
use crate::responses::seven_response::SevenGameStartResponse;
use crate::responses::thirty_one_response::ThirtyOneGameStartResponse;
use crate::responses::{EmitContext, Event, Planned};
use crate::{
    objects::{GameLogic, game7::Game7, states::SharedState},
    responses::Responses,
    socket::send_error_socket::Error,
};
use serde::{Deserialize, Serialize};
use socketioxide::{SocketIo, extract::SocketRef};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartGamePayload {}

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
        GameLogic::Lobby(lobby_logic) => {
            let game_mode = lobby_logic.settings.get_gamemode();

            match game_mode {
                GameModeSettings::Game7 { bot_count: _ } => 'match_game_mode: {
                    // Build the new game
                    let new_game = Game7::from(lobby_logic.to_owned());
                    // Move it into the guard
                    *game_guard = GameLogic::Game7(new_game);

                    // Borrow it back immutably
                    let GameLogic::Game7(ref game7_ref) = *game_guard else {
                        break 'match_game_mode Err(Error::LobbyError(
                            "Failed to assign new Game7".into(),
                        ));
                    };

                    // Build responses for all players
                    let responses = game7_ref
                        .players
                        .iter()
                        .map(|player| {
                            Planned::new(
                                Event::StartedGame,
                                EmitContext::SingleString {
                                    io: &io,
                                    sid: player.id.to_string(),
                                },
                                &SevenGameStartResponse::from((player.id.as_str(), game7_ref)),
                            )
                        })
                        .collect();
                    Ok(Responses::Multiple(responses))
                }
                GameModeSettings::Game31 { bot_count: _ } => 'match_game_mode: {
                    let new_game = Game31::from(lobby_logic.to_owned());
                    *game_guard = GameLogic::Game31(new_game);

                    let GameLogic::Game31(ref game31_ref) = *game_guard else {
                        break 'match_game_mode Err(Error::LobbyError(
                            "Failed to assign new Game31".into(),
                        ));
                    };

                    let responses = game31_ref
                        .get_players()
                        .iter()
                        .map(|p| {
                            Planned::new(
                                Event::StartedGame,
                                EmitContext::SingleString {
                                    io: &io,
                                    sid: p.id.to_string(),
                                },
                                &ThirtyOneGameStartResponse::from((p.id.as_str(), game31_ref)),
                            )
                        })
                        .collect();
                    Ok(Responses::Multiple(responses))
                }
                GameModeSettings::Game500 { bot_count: _ } => {
                    Err(Error::LobbyError("Game mode are not done".into()))
                }
                GameModeSettings::PlanningPoker { bot_count: _ } => 'match_game_mode: {
                    let new_game = PlanningPoker::from(lobby_logic.to_owned());
                    *game_guard = GameLogic::PlanningPoker(new_game);

                    let GameLogic::PlanningPoker(ref planning_poker_ref) = *game_guard else {
                        break 'match_game_mode Err(Error::LobbyError(
                            "Failed to assign new Planning Poker".into(),
                        ));
                    };

                    let responses = planning_poker_ref
                        .get_players()
                        .iter()
                        .map(|_p| {
                            Planned::new(
                                Event::StartedGame,
                                EmitContext::Room {
                                    io: &io,
                                    room_id: planning_poker_ref.get_id(),
                                },
                                &PlanningPokerResponse::from(&*planning_poker_ref),
                            )
                        })
                        .collect();
                    Ok(Responses::Multiple(responses))
                }
                GameModeSettings::Unselected {} => Err(Error::LobbyError(
                    "You can not play the game Unselected... 🐬".into(),
                )),
            }
        }
        _ => Err(Error::LobbyError("Expected Lobby before starting".into())),
    };

    match result {
        Err(e) => e.emit_error_response(&s),
        Ok(responses) => responses.emit_all(),
    }
}
