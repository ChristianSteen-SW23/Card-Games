use std::vec;

use crate::objects::lobby::lobby_player::PlayerLobby;

#[derive(Debug, Clone)]
pub struct PlayerPlanningPoker {
    pub id: String,
    pub name: String,
    pub is_bot: bool,
    pub card_played: Option<u32>,
}

impl PlayerPlanningPoker {
    pub fn reset(&mut self) {
        self.card_played = None;
    }
}

impl From<&PlayerLobby> for PlayerPlanningPoker {
    fn from(p: &PlayerLobby) -> Self {
        PlayerPlanningPoker {
            id: p.id.to_owned(),
            name: p.name.to_owned(),
            is_bot: p.is_bot,
            card_played: None,
        }
    }
}
