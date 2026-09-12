use serde::{Deserialize, Serialize};

use crate::objects::lobby::lobby_player::PlayerLobby;


#[derive(Serialize, Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub playerid: String,
    pub name: String,
    pub host: bool,
}

impl From<(&PlayerLobby,bool)> for PlayerResponse {
    fn from(value: (&PlayerLobby,bool)) -> Self {
        let (player,is_host) = value;
        Self {
            playerid: player.id.to_string(),
            name: player.name.to_string(),
            host: is_host,
        }
    }
}