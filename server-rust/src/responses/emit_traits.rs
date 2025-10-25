use std::fmt::Debug;

use serde::Serialize;
use socketioxide::{SocketIo, extract::SocketRef};

pub trait EmitSingle {
    fn emit_single(&self, s: &SocketRef, event_name: String)
    where
        Self: Serialize,
        Self: Debug,
    {
        println!("{:?}",self);
        if let Err(err) = s.emit(event_name, self) {
            println!("SendError: {:?}", err);
        }
    }
}

pub trait EmitAll {
    fn emit_all(&self, room_id: u32, io: &SocketIo, event_name: String)
    where
        Self: Serialize,
        Self: Debug,
    {
        println!("{:?}",self);
        if let Err(err) = io
            .within(room_id.to_string())
            .broadcast()
            .emit(event_name, self)
        {
            println!("SendError: {:?}", err);
        }
    }
}
