use serde::{Deserialize, Serialize};

use crate::{
    objects::lobby::{lobby::Lobby, settings_lobby::SettingsLobby},
    responses::PlayerResponse,
};

#[derive(Serialize, Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LobbyResponse {
    pub id: u32,
    pub players: Vec<PlayerResponse>,
    pub lobby_settings: SettingsLobby,
}

impl From<&Lobby> for LobbyResponse {
    fn from(value: &Lobby) -> Self {
        let host = value.get_host();
        Self {
            id: value.get_game_id(),
            players: value
                .get_players()
                .get_all()
                .iter()
                .map(|player| (player, host == player.get_id()).into())
                .collect(),
            lobby_settings: value.get_settings().clone(),
        }
    }
}
