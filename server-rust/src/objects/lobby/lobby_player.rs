use crate::socket::send_error_socket::Error;

#[derive(Debug, Clone)]
pub struct PlayerLobby {
    pub id: String,
    pub name: String,
    pub is_bot: bool,
}

impl PlayerLobby {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name, is_bot: false }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn username_check(username: &str) -> bool {
        username.len() >= 2
    }

}

impl TryFrom<(String, String)> for PlayerLobby {
    type Error = Error;

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        let (sid, username) = value;

        if !PlayerLobby::username_check(&username) {
            return Err(Error::LobbyError("Your username did not uphold the high standards required".to_string()));
        }

        Ok(PlayerLobby::new(sid, username))
    }
}

