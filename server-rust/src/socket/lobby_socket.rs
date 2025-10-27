use colored::Colorize;
use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use socketioxide::{SocketIo, extract::SocketRef};

use crate::{
    objects::{GameLogic, LobbyLogic, lobby_logic, states::SharedState},
    responses::{
        self, LobbyResponse, Response, lobby_response::LobbyAction, responses_enum::Responses,
    },
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

pub fn lobby_controller(
    s: SocketRef,
    io: SocketIo,
    payload_data: LobbyPayload,
    mut state: SharedState,
) {
    println!(
        "Lobby Event:{} {}",
        format!("{:?}", payload_data).blue(),
        s.id.to_string().green()
    );
    let mut lobby_id: u32 = 0;

    let result: Result<Responses, Error> = match payload_data.event_type {
        LobbyEvents::JoinLobby => {
            join_lobby(s.id.to_string(), payload_data, &mut state).and_then(|(id, res)| {
                let _ = s.join(id.to_string());
                lobby_id = id;
                Ok(res)
            })
        }
        LobbyEvents::CreateLobby => make_lobby(s.id.to_string(), payload_data, &mut state)
            .and_then(|id| -> Result<Responses, Error> {
                let _ = s.join(id.to_string());
                lobby_id = id;

                let state = state.lock().unwrap();

                let lobby_arc = state.game_map.get(&id).unwrap();

                let lobby = lobby_arc.lock().unwrap();

                let GameLogic::LobbyLogic(ref lobby) = *lobby;

                Ok(Responses::Single(Response::Lobby((
                    LobbyResponse::from(lobby),
                    LobbyAction::Join,
                ))))

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
        Ok(value) => value.emit_ok_response(&s, &io, lobby_id),
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

fn join_lobby(
    sid: String,
    payload_data: LobbyPayload,
    state: &mut SharedState,
) -> Result<(u32, Responses), Error> {
    let lobby_id = payload_data
        .lobby_id
        .ok_or_else(|| Error::LobbyError("You need to send a ID".to_string()))?;

    let mut state_guard = state.lock().unwrap();
    let res: LobbyResponse;
    {
        let lobby_arc = state_guard
            .game_map
            .get(&lobby_id)
            .ok_or_else(|| Error::LobbyError(format!("Lobby with ID {} not found", lobby_id)))?;

        let mut lobby_guard = lobby_arc.lock().unwrap();
        let lobby = match &mut *lobby_guard {
            GameLogic::LobbyLogic(lobby) => lobby,
            _ => return Err(Error::LobbyError("Game is not a lobby".into())),
        };
        lobby.add_player(&sid, payload_data.username)?;
        res = LobbyResponse::from(&*lobby).to_owned();
    }

    state_guard.insert_player_lobby(sid, lobby_id);

    Ok((
        lobby_id,
        Responses::Multiple(vec![
            Response::Lobby((res.clone(), LobbyAction::Join)),
            Response::Lobby((res, LobbyAction::Update)),
        ]),
    ))
}
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
