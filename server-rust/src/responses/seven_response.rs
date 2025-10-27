use serde::{Deserialize, Serialize};

use crate::responses::{EmitAll, EmitSingle, TurnResponse};

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
impl EmitAll for SevenGameUpdateResponse {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SevenPlayerResponse{
    pub name: String,
    pub id: String,
    pub cards_left: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hand7GameResponse {
    pub hand_info: Vec<u32>,
}

impl EmitSingle for Hand7GameResponse{}

