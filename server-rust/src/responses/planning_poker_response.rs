use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::objects::planning_poker::{
    game_planning_poker::PlanningPoker, player_planning_poker::PlayerPlanningPoker,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlanningPokerResponse {
    pub players_info: Vec<ThirtyOnePlayerResponse>,
    pub game_over: bool,
    pub card_set: HashMap<u32, String>,
    pub game_mode: String,
}

impl From<&PlanningPoker> for PlanningPokerResponse {
    fn from(pp_logic: &PlanningPoker) -> Self {
        let players_info = pp_logic
            .get_players()
            .iter()
            .map(ThirtyOnePlayerResponse::from)
            .collect();

        Self {
            players_info,
            game_over: pp_logic.get_game_over(),
            card_set: pp_logic.get_card_set().clone(),
            game_mode: String::from("Planning Poker"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ThirtyOnePlayerResponse {
    pub picked: Option<u32>,
    pub name: String,
}

impl From<&PlayerPlanningPoker> for ThirtyOnePlayerResponse {
    fn from(p: &PlayerPlanningPoker) -> Self {
        Self {
            picked: p.card_played,
            name: p.name.to_string(),
        }
    }
}
