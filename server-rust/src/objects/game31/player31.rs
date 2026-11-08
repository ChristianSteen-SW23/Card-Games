use std::vec;

use crate::objects::lobby::lobby_player::PlayerLobby;

#[derive(Debug, Clone)]
pub struct Player31 {
    pub id: String,
    pub name: String,
    pub is_bot: bool,
    pub has_knocked: bool,
    pub hand: Vec<u32>,
    pub total_wins: u32,
    pub cur_score: u32,
}

impl Player31 {
    pub fn reset(&mut self) {
        self.hand = vec![];
        self.has_knocked = false;
        self.cur_score = 0;
    }

    pub fn play_again(&mut self) {
        self.hand.clear();
    }

    pub fn count_points_hand(&mut self) -> &Player31 {
        let mut suit_points = [0u32; 4];

        for &card in &self.hand {
            let rank = card % 13 + 1;
            let suit = (card / 13) as usize;

            suit_points[suit] += match rank {
                1 => 11,
                10..=13 => 10,
                _ => rank,
            };
        }

        self.cur_score = suit_points.into_iter().max().unwrap_or(0);
        self
    }
}

impl From<&PlayerLobby> for Player31 {
    fn from(p: &PlayerLobby) -> Self {
        Player31 {
            id: p.id.to_owned(),
            name: p.name.to_owned(),
            is_bot: p.is_bot,
            hand: vec![],
            total_wins: 0,
            has_knocked: false,
            cur_score: 0,
        }
    }
}
