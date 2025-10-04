use serde::Serialize;

use crate::models::Player;


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnResponse {
    next: String,
    current: String
}

#[derive(Debug, Clone)]
pub struct TurnManager {
    current: Option<String>,
    next: Option<String>,
}

impl TurnManager {
   pub fn make_respone(&self)-> TurnResponse{
        TurnResponse { next: self.get_next_owned(), current: self.get_current_owned() }
    }

    pub fn new() -> Self {
        Self { current: None, next: None }
    }

    pub fn set_current(&mut self, player_id: String) {
        self.current = Some(player_id);
    }

    pub fn set_next(&mut self, players: &[Player]) {
        self.next = self.get_next(players);
    }

    pub fn get_current(&self) -> Option<&str> {
        self.current.as_deref()
    }
    
    pub fn get_current_owned(&self) -> String {
        self.current.as_ref().unwrap().clone()
    }  

    pub fn get_next(&self, players: &[Player]) -> Option<String> {
        let current_id = self.current.as_ref()?;
        let current_index = players.iter().position(|p| &p.id == current_id)?;
        let next_index = (current_index + 1) % players.len();
        Some(players[next_index].id.clone())
    }

    pub fn get_next_owned(&self) -> String {
        self.next.as_ref().unwrap().clone()
    }

    pub fn advance_turn(&mut self, players: &[Player]) {
        if let Some(next_id) = self.next.take() {
            self.current = Some(next_id);
            self.set_next(players);
        }
    }

 
}
