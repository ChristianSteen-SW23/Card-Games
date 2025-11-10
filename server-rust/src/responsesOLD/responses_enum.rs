use serde::Serialize;
use socketioxide::{SocketIo, extract::SocketRef};

use crate::responses::{EmitAll, EmitSingle, LobbyResponse, SevenGameAction, lobby_response::LobbyAction};

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
    SevenGame(SevenGameAction)
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
            Response::SevenGame(action) => {
                use SevenGameAction::*;
                match action{
                    GameStart(res) => res.emit_single(s, "startedGame".to_string()),
                    Update(res) => res.emit_all(room_id, io, "gameInfo".to_string()),
                    Hand(res) => res.emit_single(s, "handInfo".to_string()),
                }
            },
        }
    }
}
