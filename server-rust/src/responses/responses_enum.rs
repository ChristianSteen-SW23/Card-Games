use socketioxide::{extract::SocketRef, SocketIo};

use crate::responses::{lobby_response::{self, LobbyAction}, LobbyResponse};



pub enum Response {
    Lobby((LobbyResponse, LobbyAction)),
}

impl Response {
    fn emit_ok_response(self, s: SocketRef, io: SocketIo) {
        match self {
            Response::Lobby((res, action)) => { 
                use LobbyAction::*;
                match action {
                    Create => res.e,
                    Join => todo!(),
                    Update => todo!(), 
                }
            },
        }
    }

    fn emit_socket() {

    }

    fn emit_room(){

    }
}

// Con_JSON --> Socket der skal connect

// information_JSON --> Alle

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