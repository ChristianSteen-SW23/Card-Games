use colored::Colorize;
use rand::Rng;
use serde::{Deserialize, Serialize};
use socketioxide::extract::SocketRef;
use tokio::io::join;
use tracing::info;

use crate::{
    models::{Lobby, Player, lobby},
    socket::send_error_socket::{ErrorResponse, send_error_message},
    state::SharedState,
};

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
pub struct PlayersResponse {
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
        format!("{:?}", data).blue(),
        s.id.to_string().green()
    );

    match data.event_type {
        LobbyEvents::JoinLobby => join_lobby(s, data, state),
        LobbyEvents::CreateLobby => create_lobby(s, data, state),
    }

    //let _ = s.emit("lobby_response", "ok");
}

fn join_lobby(s: SocketRef, data: LobbyPayload, state: SharedState) {
    let Some(lobby_id) = data.lobby_id else {
        send_error_message(
            &s,
            ErrorResponse {
                message: "Missing lobby_id".to_string(),
                type_message: "Lobby".to_string(),
            },
        );
        return;
    };

    let mut state = state.lock().unwrap();
    let lobby_arc = match state.lobbies.get(&lobby_id) {
        Some(lobby_arc) => lobby_arc.clone(),
        None => {
            return send_error_message(
                &s,
                ErrorResponse {
                    message: "Lobby code does not exist".to_string(),
                    type_message: "Lobby".to_string(),
                },
            );
        }
    };

    let mut lobby = lobby_arc.lock().unwrap();

    if lobby.game_started {
        return send_error_message(
            &s,
            ErrorResponse {
                message: "The lobby has already started their game".to_string(),
                type_message: "Lobby".to_string(),
            },
        );
    }

    let Some(username) = &data.username else {
        return send_error_message(
            &s,
            ErrorResponse {
                message: "Missing username".to_string(),
                type_message: "Lobby".to_string(),
            },
        );
    };

    if !check_username(&s, username) {
        return send_error_message(
            &s,
            ErrorResponse {
                message: "Invalid username".to_string(),
                type_message: "Lobby".to_string(),
            },
        );
    }

    s.join(lobby_id.to_string()).ok();

    lobby.add_player(Player {
        id: s.id.to_string(),
        name: username.to_string(),
    });
    state.add_player_lobby(lobby_id, s.id.to_string());

    let response = lobby.to_response();
    let response2 = lobby.to_response();

    // broadcast to everyone else in the room
    s.within(lobby_id.to_string()).broadcast()
        .emit(
            "playerHandler",
            PlayersResponse {
                players: response2.players,
            },
        )
        .ok();

    let _ = s.emit("conToLobby", &response);
}

fn create_lobby(s: SocketRef, data: LobbyPayload, state: SharedState) {
    let mut state = state.lock().unwrap(); // lock global state

    let new_id = create_lobby_id();
    let mut lobby = Lobby::new(new_id, s.id.to_string());
    let username_opt = data.username.clone();

    match data.username {
        Some(username) => {
            if !check_username(&s, &username.clone()) {
                return;
            }

            let player = Player::new(s.id.to_string(), username.clone());
            lobby.add_player(player);

            state.add_lobby(lobby);
            state.add_player_lobby(new_id, s.id.to_string());

            let response = LobbyResponse {
                id: new_id,
                players: vec![PlayerResponse {
                    playerid: s.id.to_string(),
                    name: username,
                    host: true,
                }],
            };
            s.join(new_id.to_string()).ok();
            let _ = s.emit("conToLobby", response);
        }
        None => send_error_message(
            &s,
            ErrorResponse {
                message: "No username given".to_string(),
                type_message: "???".to_string(),
            },
        ),
    }
}

pub fn check_username(s: &SocketRef, username: &str) -> bool {
    if username.len() < 2 {
        send_error_message(
            s,
            ErrorResponse {
                message: "Invalid username".to_string(),
                type_message: "Lobby".to_string(),
            },
        );
        false
    } else {
        true
    }
}

pub fn create_lobby_id() -> u32 {
    let mut rng = rand::thread_rng();

    loop {
        let id: u32 = rng.gen_range(0..10); // 0–9
        // TODO: check against Rooms here
        break id;
    }
}
