use crate::models::Player;

#[derive(Debug, Clone)]
pub struct TurnManager {
    current: Option<String>,
    next: Option<String>,
}

impl TurnManager {
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

    pub fn get_next(&self, players: &[Player]) -> Option<String> {
        let current_id = self.current.as_ref()?;
        let current_index = players.iter().position(|p| &p.id == current_id)?;
        let next_index = (current_index + 1) % players.len();
        Some(players[next_index].id.clone())
    }

    pub fn advance_turn(&mut self, players: &[Player]) {
        if let Some(next_id) = self.next.take() {
            self.current = Some(next_id);
            self.set_next(players);
        }
    }
}
