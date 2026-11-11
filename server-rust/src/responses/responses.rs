use colored::Colorize;
use serde::Serialize;
use serde_json::Value;
use socketioxide::socket::Sid;

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
                println!(
                    "{}",
                    format!(
                        "[single_ref emit] s: {} | evnet: {}",
                        s.ns().to_string(),
                        self.event.as_str()
                    )
                    .bright_black()
                );
                if let Err(err) = s.emit(self.event.as_str(), &self.data) {
                    eprintln!("SendError(single): {:?}", err);
                }
            }
            EmitContext::Room { io, room_id } => {
                println!(
                    "{}",
                    format!(
                        "[room emit] room: {} | event: {} \n {}",
                        room_id,
                        self.event.as_str(),
                        serde_json::to_string_pretty(&self.data).unwrap_or_default()
                    )
                    .bright_black()
                );
                let io = io.clone();
                let room = room_id.to_string();
                let event = self.event.as_str().to_string();
                let data = self.data;

                tokio::spawn(async move {
                    if let Err(err) = io.within(room).broadcast().emit(event, &data).await {
                        eprintln!("SendError(room): {:?}", err);
                    }
                });
            }
            EmitContext::SingleString { io, sid } => {
                println!(
                    "{}",
                    format!(
                        "[single_string emit] s: {} | evnet: {}",
                        sid,
                        self.event.as_str()
                    )
                    .bright_black()
                );
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
#[derive(Debug)]
///
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
