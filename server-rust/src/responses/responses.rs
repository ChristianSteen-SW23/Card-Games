use serde::Serialize;
use serde_json::Value;
use socketioxide::{extract::SocketRef, socket::Sid};

use crate::responses::{EmitContext, Event};
use std::{fmt::Debug, str::FromStr};

#[derive(Debug)]
pub struct Planned<'c> {
    event: Event,
    ctx: EmitContext<'c>,
    data: Value,
}

impl<'c> Planned<'c> {
    pub fn new<T: Serialize + Debug>(event: Event, ctx: EmitContext<'c>, payload: &T) -> Self {
        Self {
            event,
            ctx,
            data: serde_json::to_value(payload).expect("serialize payload"),
        }
    }

    pub fn emit(self) {
        match self.ctx {
            EmitContext::SingleRef { s } => {
                if let Err(err) = s.emit(self.event.as_str(), &self.data) {
                    eprintln!("SendError(single): {:?}", err);
                }
            }
            EmitContext::Room { io, room_id } => {
                if let Err(err) = io
                    .within(room_id.to_string())
                    .broadcast()
                    .emit(self.event.as_str(), &self.data)
                {
                    eprintln!("SendError(room): {:?}", err);
                }
            }
            EmitContext::SingleString { io, sid } => {
                // Parse the socket id string into the library’s type safely
                let sid = match Sid::from_str(&sid) {
                    Ok(id) => id,
                    Err(_) => {
                        eprintln!("Invalid socket id '{}'", sid);
                        return;
                    }
                };

                // Get the default namespace, skip if not found
                let Some(ns) = io.of("/") else {
                    eprintln!("Namespace '/' not found");
                    return;
                };

                // Get the socket handle, skip if missing
                let Some(target) = ns.get_socket(sid) else {
                    eprintln!("Socket {} not found", sid);
                    return;
                };

                // Finally emit
                if let Err(err) = target.emit(self.event.as_str(), &self.data) {
                    eprintln!("SendError (SingleString): {:?}", err);
                }
            }
        }
    }
}

/// Keep your “single/multiple” envelope API
pub enum Responses<'c> {
    Single(Planned<'c>),
    Multiple(Vec<Planned<'c>>),
}

impl<'c> Responses<'c> {
    pub fn emit_all(self) {
        match self {
            Responses::Single(p) => p.emit(),
            Responses::Multiple(list) => {
                for p in list {
                    p.emit();
                }
            }
        }
    }
}
