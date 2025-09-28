use colored::Colorize;
use serde::{Deserialize, Serialize};
use socketioxide::extract::SocketRef;
use tracing::info;
use rand::Rng;

use crate::{models::{lobby, Lobby, Player}, state::SharedState};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LobbyPayload {
    pub lobby_id: Option<u32>,
    pub username: Option<String>,
    pub event_type: LobbyEvents,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LobbyEvents {
    JoinLobby,
    CreateLobby,
}

#[derive(Debug, Serialize)]
pub struct LobbyResponse {
    pub id: u32,
    pub players: Vec<PlayerResponse>,
}

#[derive(Debug, Serialize)]
pub struct PlayerResponse {
    pub playerid: String,
    pub name: String,
    pub host: bool,
}

pub fn lobby_controller(s: SocketRef, data: LobbyPayload, state: SharedState) {
    println!(
        "Lobby Event:{} {}",
        format!("{:?}", data).blue(), s.id.to_string().green()
    );

    match data.event_type {
        LobbyEvents::JoinLobby => todo!(),
        LobbyEvents::CreateLobby => create_lobby(s, data, state),
    }

    //let _ = s.emit("lobby_response", "ok");
}

fn create_lobby(s: SocketRef, data: LobbyPayload, state: SharedState){
    let mut state = state.lock().unwrap(); // lock global state
    let new_id = create_lobby_id();
    let mut lobby = Lobby::new(new_id, s.id.to_string());
    let username_opt = data.username.clone();
    if let Some(name) = username_opt {
        let player = Player::new(s.id.to_string(), name);
        lobby.add_player(player);
    }

    state.add_lobby(lobby);

    let response = LobbyResponse {
        id: new_id,
        players: vec![PlayerResponse {
            playerid: s.id.to_string(),
            name: data.username.unwrap_or_default(),
            host: true,
        }],
    };
    let _ = s.emit("conToLobby", response);
}


pub fn create_lobby_id() -> u32 {
    let mut rng = rand::thread_rng();

    loop {
        let id: u32 = rng.gen_range(0..10); // 0–9
        // TODO: check against Rooms here
        break id;
    }
}
