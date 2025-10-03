use crate::models::{PlayerGameData};

#[derive(Debug, Clone)]
pub struct Player7Data {
    hand: Vec<u32>,
    total_score: u32,
}

impl PlayerGameData for Player7Data {
    fn clone_box(&self) -> Box<dyn PlayerGameData + Send + Sync> {
        Box::new(self.clone())
    }
}