use crate::{
    objects::{lobby::{lobby_player::PlayerLobby, lobby_players::PlayersLobby}, traits::has_players::HasPlayers}, socket::{LobbyPayload, send_error_socket::Error},
};

#[derive(Debug, Clone)]
pub struct Lobby {
    pub id: u32,
    pub host: String,
    pub players: PlayersLobby,
}

impl Lobby {
    pub fn get_game_id(&self) -> u32 {
        self.id
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_players(&self) -> &PlayersLobby {
        &self.players
    }

    pub fn add_player(&mut self, sid: &String, username: Option<String>) -> Result<(), Error> {
        let player = PlayerLobby::try_from((
            sid.to_string(),
            username.ok_or_else(|| Error::LobbyError("You need to send a username".to_string()))?,
        ))?;
        self.players.add(player);
        Ok(())
    }

    pub fn remove_player(&mut self, sid: &String) {
        self.players.remove(sid);
        if self.get_host() == sid && self.players.0.len() > 0{
            self.host = self.player_ids()[0].to_string();
        }
    }
}

impl TryFrom<(LobbyPayload, u32, String)> for Lobby {
    type Error = Error;

    fn try_from(value: (LobbyPayload, u32, String)) -> Result<Self, Self::Error> {
        let (lobby_payload, new_lobby_id, sid) = value;
        let username = lobby_payload
            .username
            .ok_or_else(|| Error::LobbyError("You need to send a username".to_string()))?;

        let player = PlayerLobby::try_from((sid.clone(), username))?;
        Ok(Self {
            id: new_lobby_id,
            host: sid,
            players: PlayersLobby::from(player)
        })
    }
}
