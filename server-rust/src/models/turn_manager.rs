use std::collections::HashMap;

use serde::Serialize;

use crate::models::{Player, Players};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnResponse {
    next: String,
    current: String,
}

#[derive(Debug, Clone)]
pub struct TurnManager {
    current: String,
    next: String,
}

impl TurnManager {
    pub fn make_respone(&self) -> TurnResponse {
        TurnResponse {
            current: self.current.clone(),
            next: self.next.clone(),
        }
    }

    pub fn new() -> Self {
        Self { current: String::new(), next: String::new() }
    }

    pub fn update(&mut self, current: String, players: &Players) -> Result<(), ()> {
        if let Some(next) = players.get_next(&current).and_then(|player| Some(player.id.to_string())) {
            self.current = current; 
            self.next = next; 
            Ok(())
        } else { Err(()) }
    }

    pub fn get_current(&self) -> &str {
        &self.current
    }

    pub fn get_next(&self) -> &str {
        &self.next
    }
    
    pub fn advance_turn(&mut self, players: &Players) -> Result<(), ()> {
        self.update(self.next.to_string(), players)
    }
}
