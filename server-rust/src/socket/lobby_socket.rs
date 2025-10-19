use colored::Colorize;
use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use socketioxide::{extract::SocketRef, SocketIo};

use crate::{
    objects::{GameLogic, LobbyLogic, lobby_logic, states::SharedState},
    responses::{LobbyResponse, Response, lobby_response::LobbyAction},
    socket::send_error_socket::Error,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LobbyPayload {
    pub lobby_id: Option<u32>,
    pub username: Option<String>,
    pub event_type: LobbyEvents,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LobbyEvents {
    JoinLobby,
    CreateLobby,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct LobbyResponse {
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

pub fn lobby_controller(s: SocketRef, io: SocketIo, payload_data: LobbyPayload, mut state: SharedState) {
    println!(
        "Lobby Event:{} {}",
        format!("{:?}", payload_data).blue(),
        s.id.to_string().green()
    );
    let mut lobby_id: u32 = 0;

    let result: Result<Response, Error> = match payload_data.event_type {
        LobbyEvents::JoinLobby => {
            // Make new player and join it to the room

            todo!();

            // let mut state = state.lock().unwrap();
            // let mut lobby_logic = state.game_map.get(state.player_lobby_map.get(s.id.to_string().as_str()).unwrap());
            // Ok(Response::Lobby((LobbyResponse::from(lobby_logic), LobbyAction::Join)))
        }
        LobbyEvents::CreateLobby => make_lobby(s.id.to_string(), payload_data, &mut state)
            .and_then(|id| -> Result<Response, Error> {
                s.join(id.to_string());
                lobby_id = id;

                let state = state.lock().unwrap();

                let lobby_arc = state.game_map.get(&id).unwrap();

                let lobby = lobby_arc.lock().unwrap();

                let GameLogic::LobbyLogic(ref lobby) = *lobby;

                Ok(Response::Lobby((
                    LobbyResponse::from(lobby),
                    LobbyAction::Create,
                )))

                // if let Some(lobby) = state
                //     .lock()
                //     .ok()
                //     .map(|state| state.game_map.get(&id))
                //     .flatten()
                //     .map(|lobby_arc| lobby_arc.lock().ok().map(|lobby| lobby))
                //     .flatten()
                // {
                //     let GameLogic::LobbyLogic(ref lobby_2) = *lobby;
                //     // let lobby_2 =

                // } else {
                //     Err(Error::LobbyError("No".to_string()))
                // }
            }),
    };

    match result {
        Err(e) => e.emit_error_response(&s),
        Ok(value) => value.emit_ok_response(s, io, lobby_id),
    }
}

/**
    Lobby is created and attached to sharedstate
    * @return The lobby id
*/
fn make_lobby(
    sid: String,
    data_payload: LobbyPayload,
    state: &mut SharedState,
) -> Result<u32, Error> {
    let new_id = create_lobby_id(&state)
        .ok_or_else(|| Error::LobbyError("Could not generate a free lobby code".to_string()))?;

    let new_lobby_logic: LobbyLogic = (data_payload, new_id, sid.to_string()).try_into()?;
    let mut locked_state = state
        .lock()
        .map_err(|err| Error::LobbyError(err.to_string()))?; // lock global state

    locked_state.insert_game(new_lobby_logic, new_id);
    locked_state.insert_player_lobby(sid, new_id);

    Ok(new_id)
}

pub fn create_lobby_id(state: &SharedState) -> Option<u32> {
    let mut rng = rng();
    let state = state.lock().unwrap();

    let max = 10;
    let id: u32 = rng.random_range(0..max);

    for i in 1..max + 1 {
        let candidate = (id + i) % max;
        if !state.game_map.contains_key(&candidate) {
            return Some(candidate);
        }
    }

    None
}

// fn create_lobby_old(s: SocketRef, data: LobbyPayload, state: SharedState) {
//     let new_id = match find_lobby_id(&state) {
//         Ok(id) => id,
//         Err(_) => {
//             return send_error_message(
//                 &s,
//                 ErrorResponse {
//                     message: "Could not generate a free lobby code".to_string(),
//                     r#type: "Lobby".to_string(),
//                 },
//             );
//         }
//     };

//     let mut state = state.lock().unwrap(); // lock global state

//     let mut lobby = GameData::new(new_id, s.id.to_string());

//     match data.username {
//         Some(username) => {
//             if !check_username(&s, &username.to_string()) {
//                 return;
//             }

//             let player = Player::new(s.id.to_string(), username.to_string());
//             lobby.add_player(player);

//             state.insert_lobby(lobby);
//             state.add_player_lobby(new_id, s.id.to_string());

//             let response = LobbyResponse {
//                 id: new_id,
//                 players: vec![PlayerResponse {
//                     playerid: s.id.to_string(),
//                     name: username,
//                     host: true,
//                 }],
//             };
//             s.join(new_id.to_string()).ok();
//             let _ = s.emit("conToLobby", response);
//         }
//         None => send_error_message(
//             &s,
//             ErrorResponse {
//                 message: "No username given".to_string(),
//                 r#type: "???".to_string(),
//             },
//         ),
//     }
// }

// fn join_lobby(s: SocketRef, data: LobbyPayload, state: SharedState) {
//     let Some(lobby_id) = data.lobby_id else {
//         return send_error_message(
//             &s,
//             ErrorResponse {
//                 message: "Missing lobby_id".to_string(),
//                 r#type: "Lobby".to_string(),
//             },
//         );
//     };

//     let mut state = state.lock().unwrap();
//     let lobby_arc = match state.lobbies.get(&lobby_id) {
//         Some(lobby_arc) => lobby_arc.clone(),
//         None => {
//             return send_error_message(
//                 &s,
//                 ErrorResponse {
//                     message: "Lobby code does not exist".to_string(),
//                     r#type: "Lobby".to_string(),
//                 },
//             );
//         }
//     };

//     let mut lobby = lobby_arc.lock().unwrap();

//     if lobby.game_started {
//         return send_error_message(
//             &s,
//             ErrorResponse {
//                 message: "The lobby has already started their game".to_string(),
//                 r#type: "Lobby".to_string(),
//             },
//         );
//     }

//     let Some(username) = &data.username else {
//         return send_error_message(
//             &s,
//             ErrorResponse {
//                 message: "Missing username".to_string(),
//                 r#type: "Lobby".to_string(),
//             },
//         );
//     };

//     if !check_username(&s, username) {
//         return send_error_message(
//             &s,
//             ErrorResponse {
//                 message: "Invalid username".to_string(),
//                 r#type: "Lobby".to_string(),
//             },
//         );
//     }

//     s.join(lobby_id.to_string()).ok();

//     lobby.add_player(Player {
//         id: s.id.to_string(),
//         name: username.to_string(),
//         game: None,
//     });
//     state.add_player_lobby(lobby_id, s.id.to_string());
//     drop(state);

//     let response = lobby.to_response();

//     s.within(lobby_id.to_string())
//         .broadcast()
//         .emit(
//             "playerHandler",
//             PlayersResponse {
//                 players: response.players.clone(),
//             },
//         )
//         .ok();

//     let _ = s.emit("conToLobby", &response);
// }
