use crate::{objects::PlayerGameData, socket::send_error_socket::Error};

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub game: PlayerGameData,
}

impl Player {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name, game: PlayerGameData::Lobby }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    // pub fn get_mut_game(&mut self) ->  Option<&mut PlayerGameData> {
    //     self.game.as_mut()
    // }
    pub fn username_check(username: &str) -> bool {
        username.len() >= 2
    }
}

impl TryFrom<(String, String)> for Player {
    type Error = Error;

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        let (sid, username) = value;

        if !Player::username_check(&username) {
            return Err(Error::LobbyError("Your username did not uphold the high standards required".to_string()));
        }

        Ok(Player::new(sid, username))
    }
}
