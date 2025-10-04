use colored::Colorize;
use rand::{rng, Rng};
use serde::{Deserialize, Serialize};
use socketioxide::{extract::SocketRef};

use crate::{
    models::{lobby, Game7Logic, GameLogic, Lobby, Player, TurnManager},
    socket::send_error_socket::{send_error_message, ErrorResponse},
    state::SharedState,
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

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Response {
//     pub id: u32,
//     pub players: Vec<PlayerResponse>,
// }

// #[derive(Debug, Serialize, Clone, Deserialize)]
// pub struct PlayersResponse {
//     pub players: Vec<PlayerResponse>,
// }

// #[derive(Debug, Serialize, Clone, Deserialize)]
// pub struct PlayerResponse {
//     pub playerid: String,
//     pub name: String,
//     pub host: bool,
// }

pub fn start_game_controller(s: SocketRef, data: StartGamePayload, state: SharedState) {
    println!(
        "Lobby Event:{} {}",
        format!("{:?}", data).blue(),
        s.id.to_string().green()
    );

    let state = state.lock().unwrap();

    if let Some(lobby_arc) = state.lobbies.get(&state.player_lobby.get(&s.id.to_string()).unwrap()) {
        let mut lobby = lobby_arc.lock().unwrap();

        if lobby.game_started {
            return send_error_message(
                &s,
                ErrorResponse {
                    message: "Game have all ready started".to_string(),
                    r#type: "???".to_string(),
                },
            );
        }

        lobby.game_started = true;
        
        match data.game_mode {
            StartGameEvents::Seven => {
                    lobby.game = Some(GameLogic::Game7Logic(Game7Logic::new(&mut lobby, s)));
                },
            StartGameEvents::ThirtyOne => todo!(),
            StartGameEvents::FiveHundred => todo!(),
            StartGameEvents::PlanningPoker => todo!(),
        }
    }
}

