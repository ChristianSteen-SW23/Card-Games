use colored::Colorize;
use serde::Deserialize;
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

    if let Some(name) = data.username {
        let player = Player::new(s.id.to_string(), name);
        lobby.add_player(player);
    }

    state.add_lobby(lobby);

    let _ = s.emit("conToLobby", new_id);
}


pub fn create_lobby_id() -> u32 {
    let mut rng = rand::thread_rng();

    loop {
        let id: u32 = rng.gen_range(0..10); // 0–9
        // TODO: check against Rooms here
        break id;
    }
}
