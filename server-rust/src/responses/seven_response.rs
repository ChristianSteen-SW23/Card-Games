use serde::{Deserialize, Serialize};

use crate::{
    objects::game7::{Game7, player7::Player7},
    responses::TurnResponse,
};

// #[derive(Serialize, Debug, Deserialize)]
// pub enum SevenGameAction {
//     GameStart(SevenGameStartResponse),
//     // Update(SevenGameUpdateResponse),
//     // Hand(SevenHandUpdateResponse),
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SevenGameUpdateResponse {
    pub board: Vec<Vec<i32>>,
    pub players_info: Vec<SevenPlayerResponse>,
    pub turn: TurnResponse,
}

impl From<&Game7> for SevenGameUpdateResponse {
    fn from(game_7_logic: &Game7) -> Self {
        let players_info = game_7_logic
            .players
            .iter()
            .map(|p| SevenPlayerResponse::from(p))
            .collect();

        Self {
            board: game_7_logic.board.clone(),
            players_info,
            turn: game_7_logic.turn_manager.make_respone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SevenGameStartResponse {
    pub hand_info: Vec<u32>,
    pub board: Vec<Vec<i32>>,
    pub players_info: Vec<SevenPlayerResponse>,
    pub turn: TurnResponse,
    pub game_mode: String,
}

impl From<(&str, &Game7)> for SevenGameStartResponse {
    fn from((sid, game_7_logic): (&str, &Game7)) -> Self {
        let player = game_7_logic
            .get_player(sid)
            .expect("Player not found in Game7Logic");

        let players_info = game_7_logic
            .players
            .iter()
            .map(|p| SevenPlayerResponse::from(p))
            .collect();

        Self {
            hand_info: player.hand.clone(),
            board: game_7_logic.board.clone(),
            players_info,
            turn: game_7_logic.turn_manager.make_respone(),
            game_mode: "7".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SevenGameEndedResponse {
    pub win_data: Vec<SevenWinDataEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SevenWinDataEntry {
    pub name: String,
    pub round_score: u32,
    pub total_score: u32,
}

impl From<(&Game7)> for SevenGameEndedResponse {
    fn from(game_7_logic: &Game7) -> Self {
        let win_data = game_7_logic
            .players
            .iter()
            .map(|p| SevenWinDataEntry {
                name: p.name.clone(),
                round_score: p.cur_score,
                total_score: p.total_score,
            })
            .collect();

        Self { win_data }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SevenPlayerResponse {
    pub name: String,
    pub id: String,
    pub cards_left: usize,
}

impl From<&Player7> for SevenPlayerResponse {
    fn from(player: &Player7) -> Self {
        Self {
            name: player.name.clone(),
            id: player.id.clone(),
            cards_left: player.cards_left,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SevenHandUpdateResponse {
    pub hand_info: Vec<u32>,
}

impl From<(&str, &Game7)> for SevenHandUpdateResponse {
    fn from((sid, game_7_logic): (&str, &Game7)) -> Self {
        let player = game_7_logic
            .get_player(sid)
            .expect("Player not found in Game7");

        Self {
            hand_info: player.hand.clone(),
        }
    }
}
