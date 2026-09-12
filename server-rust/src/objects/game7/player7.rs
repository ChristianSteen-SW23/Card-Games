use std::vec;

use crate::objects::lobby::lobby_player::PlayerLobby;


#[derive(Debug, Clone)]
pub struct Player7 {
    pub id: String,
    pub name: String,
    pub is_bot: bool,
    pub hand: Vec<u32>,
    pub total_score: u32,
    pub cards_left: usize,
}

impl Player7 {
    pub fn reset(&mut self){
        self.hand = vec![];
        self.cards_left = 0;
    }

    pub fn set_cards_left(&mut self){
        self.cards_left = self.hand.len();
    }

    pub fn play_again(&mut self){
        self.cards_left = 0;
        self.hand.clear();
    }

    pub fn count_and_reset_hand(&mut self, with_box: bool) {
        self.total_score += self.hand.iter().map(|e| {
            match e % 13 {
                0 => 15,
                13|12|11|10 => 10,
                _ => 5
            }
        }).sum::<u32>();
        if with_box {
            self.total_score += 25;
        }
        self.reset();
    }
}

impl From <&PlayerLobby> for Player7 {
    fn from(p: &PlayerLobby) -> Self {
        Player7 {
            id: p.id.to_owned(),
            name: p.name.to_owned(),
            is_bot: p.is_bot,
            hand: vec![],
            total_score: 0,
            cards_left: 0,
        }
    }
}