use crate::models::PlayerGameData;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub game: Option<PlayerGameData>,
}

impl Player {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name, game: None }
    }

    pub fn get_mut_game(&mut self) ->  Option<&mut PlayerGameData> {
        self.game.as_mut()
    } 

}
