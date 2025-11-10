use serde::Serialize;
use socketioxide::{SocketIo, extract::SocketRef};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    ConnectToLobby,
    UpdateLobby,
    StartedGame,
    GameInfo,
    HandInfo,
}
impl Event {
    pub fn as_str(self) -> &'static str {
        match self {
            Event::ConnectToLobby => "conToLobby",
            Event::UpdateLobby => "playerHandler",
            Event::StartedGame => "startedGame",
            Event::GameInfo => "gameInfo",
            Event::HandInfo => "handInfo",
        }
    }
}

#[derive(Debug)]
pub enum EmitContext<'a> {
    SingleRef { s: &'a SocketRef },
    SingleString { io: &'a SocketIo, sid: String }, // hook up a sid -> SocketRef map if you need this
    Room { io: &'a SocketIo, room_id: u32 },
}

// impl<'a> EmitContext<'a> {
//     pub fn single(s: &'a SocketRef) -> Self {
//         Self::SingleRef { s }
//     }
//     pub fn room(io: &'a SocketIo, room_id: u32) -> Self {
//         Self::Room { io, room_id }
//     }
// }
