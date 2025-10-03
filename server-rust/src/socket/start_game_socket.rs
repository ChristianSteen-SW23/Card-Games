use colored::Colorize;
use rand::{rng, Rng};
use serde::{Deserialize, Serialize};
use socketioxide::{extract::SocketRef};

use crate::{
    models::{Lobby, Player},
    socket::send_error_socket::{ErrorResponse, send_error_message},
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

    match data.game_mode {
        StartGameEvents::Seven => todo!(),
        StartGameEvents::ThirtyOne => todo!(),
        StartGameEvents::FiveHundred => todo!(),
        StartGameEvents::PlanningPoker => todo!(),
    }
}

