use serde::{Deserialize, Serialize};

use crate::objects::Player;

#[derive(Serialize, Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub playerid: String,
    pub name: String,
    pub host: bool,
}

impl From<(&Player, bool)> for PlayerResponse {
    fn from(value: (&Player, bool)) -> Self {
        let (player, is_host) = value;
        Self {
            playerid: player.id.to_string(),
            name: player.name.to_string(),
            host: is_host,
        }
    }
}