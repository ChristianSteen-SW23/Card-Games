
use socketioxide::{extract::SocketRef, SocketIo};

use crate::responses::{lobby_response::LobbyAction, EmitSingle, EmitAll, LobbyResponse};



pub enum Response {
    Lobby((LobbyResponse, LobbyAction)),
}

impl Response {
    pub fn emit_ok_response(self, s: SocketRef, io: SocketIo, room_id: u32) {
        match self {
            Response::Lobby((res, action)) => { 
                use LobbyAction::*;
                match action {
                    Create => res.emit_single(s, "conToLobby".to_string()),
                    Update => todo!(), 
                    Join => {
                        res.emit_single(s, "conToLobby".to_string());
                        res.emit_all(room_id, io, "playerHandler".to_string());
                    },
                }
            },
        }
    }
}
