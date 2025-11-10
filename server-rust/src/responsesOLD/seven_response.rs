use serde::{Deserialize, Serialize};

use crate::{
    objects::{Game7Logic, Player, PlayerGameData, game7::game_7_logic, player},
    responses::{EmitAll, EmitSingle, TurnResponse},
};

#[derive(Serialize, Debug, Deserialize)]
pub enum SevenGameAction {
    GameStart(SevenGameStartResponse),
    Update(SevenGameUpdateResponse),
    Hand(Hand7GameResponse),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SevenGameUpdateResponse {
    pub board: Vec<Vec<i32>>,
    pub players_info: Vec<SevenPlayerResponse>,
    pub turn: TurnResponse,
}
impl EmitAll for SevenGameUpdateResponse {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SevenGameStartResponse {
    pub hand_info: Vec<u32>,
    pub board: Vec<Vec<i32>>,
    pub players_info: Vec<SevenPlayerResponse>,
    pub turn: TurnResponse,
    pub game_mode: String,
}

impl EmitSingle for SevenGameStartResponse {}

impl From<(&str, &Game7Logic)> for SevenGameStartResponse {
    fn from((sid, game_7_logic): (&str, &Game7Logic)) -> Self {
        let player = game_7_logic
            .game_data
            .players
            .get(sid)
            .expect("Player not found in Game7Logic");

        let hand_info = match &player.game {
            PlayerGameData::Player7(data) => data.hand.clone(),
            _ => Vec::new(),
        };

        let players_info = game_7_logic
            .game_data
            .players
            .get_all().iter()
            .map(|p| SevenPlayerResponse::from(p))
            .collect();

        Self {
            hand_info,
            board: game_7_logic.board.clone(),
            players_info,
            turn: game_7_logic.turn_manager.make_respone(),
            game_mode: "7".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SevenPlayerResponse {
    pub name: String,
    pub id: String,
    pub cards_left: usize,
}

impl From<&Player> for SevenPlayerResponse {
    fn from(player: &Player) -> Self {
        let cards_left = match &player.game {
            PlayerGameData::Player7(data) => data.cards_left,
            _ => 0,
        };

        Self {
            name: player.name.clone(),
            id: player.id.clone(),
            cards_left,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hand7GameResponse {
    pub hand_info: Vec<u32>,
}

impl EmitSingle for Hand7GameResponse {}
