use serde::Serialize;
use socketioxide::{SocketIo, extract::SocketRef};

use crate::responses::{EmitAll, EmitSingle, LobbyResponse, lobby_response::LobbyAction};

#[derive(Serialize, Debug)]
pub enum Responses {
    Single(Response),
    Multiple(Vec<Response>),
}

impl Responses {
    pub fn emit_ok_response(self, s: &SocketRef, io: &SocketIo, room_id: u32) {
        match self {
            Responses::Single(response) => response.emit_ok_response(s, io, room_id),
            Responses::Multiple(responses) => {
                for res in responses {
                    res.emit_ok_response(s, io, room_id);
                }
            }
        }
    }
}

#[derive(Serialize, Debug)]
pub enum Response {
    Lobby((LobbyResponse, LobbyAction)),
}

impl Response {
    pub fn emit_ok_response(self, s: &SocketRef, io: &SocketIo, room_id: u32) {
        match self {
            Response::Lobby((res, action)) => {
                use LobbyAction::*;
                match action {
                    Join => res.emit_single(s, "conToLobby".to_string()),
                    Update => res.emit_all(room_id, io, "playerHandler".to_string()),
                }
            }
        }
    }
}
