#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ThirtyOneGameUpdateResponse {
    pub top_stack_card: u32,
    pub players_info: Vec<ThirtyOnePlayerResponse>,
    pub turn: TurnResponse,
}

impl From<&Game31> for ThirtyOneGameUpdateResponse {
    fn from(game_31_logic: &Game31) -> Self {
        let players_info = game_31_logic
            .get_players()
            .iter()
            .map(|p| ThirtyOnePlayerResponse::from(p))
            .collect();

        Self {
            players_info,
            turn: game_31_logic.get_turn_manager().make_respone(),
            top_stack_card: game_31_logic.get_top_stack_card(),
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::{objects::game31::{Game31, player31::Player31}, responses::TurnResponse};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThirtyOneGameStartResponse {
    pub hand_info: Vec<u32>,
    pub top_stack_card: u32,
    pub players_info: Vec<ThirtyOnePlayerResponse>,
    pub turn: TurnResponse,
    pub game_mode: String,
}

impl From<(&str, &Game31)> for ThirtyOneGameStartResponse {
    fn from((sid, game_31_logic): (&str, &Game31)) -> Self {
        let player = game_31_logic
            .get_player(sid)
            .expect("Player not found in Game31Logic");

        let players_info = game_31_logic
            .get_players()
            .iter()
            .map(|p| ThirtyOnePlayerResponse::from(p))
            .collect();

        Self {
            hand_info: player.hand.clone(),
            top_stack_card: game_31_logic.get_top_stack_card(),
            players_info,
            turn: game_31_logic.get_turn_manager().make_respone(),
            game_mode: String::from("31"),
        }
    }
}

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// #[serde(rename_all = "camelCase")]
// pub struct ThirtyOneGameEndedResponse {
//     pub win_data: Vec<ThirtyOneWinDataEntry>,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// #[serde(rename_all = "camelCase")]
// pub struct ThirtyOneWinDataEntry {
//     pub name: String,
//     pub round_score: u32,
//     pub total_score: u32,
// }

// impl From<&Game31> for ThirtyOneGameEndedResponse {
//     fn from(game_31_logic: &Game31) -> Self {
//         let win_data = game_31_logic
//             .players
//             .iter()
//             .map(|p| ThirtyOneWinDataEntry {
//                 name: p.name.clone(),
//                 round_score: p.cur_score,
//                 total_score: p.total_score,
//             })
//             .collect();

//         Self { win_data }
//     }
// }

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ThirtyOnePlayerResponse {
    pub name: String,
    pub id: String,
}

impl From<&Player31> for ThirtyOnePlayerResponse {
    fn from(player: &Player31) -> Self {
        Self {
            name: player.name.clone(),
            id: player.id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThirtyOneHandUpdateResponse {
    pub hand_info: Vec<u32>,
}

impl From<(&str, &Game31)> for ThirtyOneHandUpdateResponse {
    fn from((sid, game_31_logic): (&str, &Game31)) -> Self {
        let player = game_31_logic
            .get_player(sid)
            .expect("Player not found in Game31");

        Self {
            hand_info: player.hand.clone(),
        }
    }
}
