use serde::{Deserialize, Serialize};
use socketioxide::{SocketIo, extract::SocketRef};
use colored::Colorize;

use crate::{objects::{GameLogic, states::SharedState}, responses::{EmitContext, Event, Planned, Responses, planning_poker_response::PlanningPokerResponse}, socket::send_error_socket::Error};


#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlanningPokerPayload {
    pub card: u32,
    pub move_type: PlanningPokerEvents,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PlanningPokerEvents {
    PlayCard,
    PlayAnotherRound,
}

pub fn planning_poker_controller(s: SocketRef, data: PlanningPokerPayload, state: SharedState, io: SocketIo) {
    println!(
        "Game 7 Event:{} {}",
        format!("{:?}", data).blue(),
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

    let planning_poker = match &mut *game_guard {
        GameLogic::PlanningPoker(planning_poker_logic) => planning_poker_logic, // &mut Game7Logic
        _ => {
            Error::LobbyError("Current gamemode is not planning poker".into()).emit_error_response(&s);
            return;
        }
    };

    let result: Result<Responses, Error> = match planning_poker.handle_move(data, &s.id.to_string()) {
        Ok(_) => {
            let mut planned = vec![
                Planned::new(
                    Event::GameInfo,
                    EmitContext::Room {
                        io: &io,
                        room_id: planning_poker.get_id(),
                    },
                    &PlanningPokerResponse::from(&*planning_poker),
                ),
            ];
            Ok(Responses::Multiple(planned))
        }
        Err(err) => Err(err),
    };

    match result {
        Err(e) => e.emit_error_response(&s),
        Ok(responses) => responses.emit_all(),
    };
}
