use std::ops::Deref;

use colored::Colorize;
use serde::{Deserialize, Serialize};
use socketioxide::{SocketIo, extract::SocketRef};

use crate::{
    objects::{GameLogic, states::SharedState},
    responses::{
        EmitContext, Event, Planned, Responses, SevenGameUpdateResponse,
        seven_response::SevenHandUpdateResponse,
    },
    socket::send_error_socket::Error,
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

pub fn game_7_controller(s: SocketRef, data: Game7Payload, state: SharedState, io: SocketIo) {
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

    let mut game7 = match &mut *game_guard {
        GameLogic::Game7Logic(game7_logic) => game7_logic, // &mut Game7Logic
        _ => {
            Error::LobbyError("Current gamemode is not 7".into()).emit_error_response(&s);
            return;
        }
    };

    let result: Result<Responses, Error> = match game7.handle_move(data, &s.id.to_string()) {
        Ok(_) => {
            Ok(Responses::Multiple(vec![
                Planned::new(
                    Event::GameInfo,
                    EmitContext::Room { io: &io, room_id: game7.game_data.id },
                    &SevenGameUpdateResponse::from(&*game7),
                ),
                Planned::new(
                    Event::HandInfo,
                    EmitContext::SingleRef { s: &s },
                    &SevenHandUpdateResponse::from((s.id.to_string().as_str(), &*game7)),
                ),
            ]))
        }
        Err(err) => Err(err),
    };

    match result {
        Err(e) => e.emit_error_response(&s),
        Ok(responses) => responses.emit_all(),
    }
}
