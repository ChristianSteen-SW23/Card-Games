use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]

pub enum GameModeSettings {
    #[serde(rename = "7")]
    Game7 {
        bot_count: u32,
    }, // (Bot count)
    #[serde(rename = "31")]
    Game31 {
        bot_count: u32,
    },
    #[serde(rename = "500")]
    Game500 {
        bot_count: u32,
    },
    #[serde(rename = "Planning Poker")]
    PlanningPoker {
        bot_count: u32,
    },
    Unselected {},
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]

pub struct SettingsLobby {
    gamemode: GameModeSettings,
    max_players: u32,
}

impl SettingsLobby {
    pub fn new() -> Self {
        Self {
            gamemode: GameModeSettings::Unselected {},
            max_players: 67,
        }
    }

    pub fn get_gamemode(&self) -> &GameModeSettings {
        &self.gamemode
    }
    pub fn set_gamemode(&mut self, gamemode: GameModeSettings) {
        self.gamemode = gamemode;
    }

    pub fn get_max_players(&self) -> u32 {
        self.max_players
    }
}
