use crate::models::PlayerGameData;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub game: Option<Box<dyn PlayerGameData + Send + Sync>>,
}

impl Player {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name, game: None }
    }
}
