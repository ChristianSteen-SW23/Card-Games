use serde::{Deserialize, Serialize};

use crate::{objects::{LobbyLogic, Players}, responses::{EmitAll, EmitSingle, PlayerResponse}};

#[derive(Serialize, Debug, Deserialize)]
pub enum LobbyAction {
    Join,
    Update,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LobbyResponse {
    pub id: u32,
    pub players: Vec<PlayerResponse>,
}

impl From<&LobbyLogic> for LobbyResponse {
    fn from(value: &LobbyLogic) -> Self {
        let host = value.get_host();
        Self {
            id: value.get_game_id(),
            players: value.get_players().get_all().iter().map(|player| (player, host == player.get_id()).into()).collect(),
        }
    }
}


impl EmitSingle for LobbyResponse {}
impl EmitAll for LobbyResponse {}
