use crate::models::Player;

pub trait GameLogic: std::fmt::Debug + Send + Sync {
    fn start_game(&mut self, players: &[Player]);
    fn handle_move(&mut self, player_id: &str, data: serde_json::Value);
    fn clone_box(&self) -> Box<dyn GameLogic + Send + Sync>;
}

impl Clone for Box<dyn GameLogic + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}