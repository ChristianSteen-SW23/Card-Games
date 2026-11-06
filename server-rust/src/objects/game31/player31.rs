use std::vec;

use crate::objects::lobby::lobby_player::PlayerLobby;


#[derive(Debug, Clone)]
pub struct Player31 {
    pub id: String,
    pub name: String,
    pub is_bot: bool,
    pub hand: Vec<u32>,
    pub total_wins: u32,
}

impl Player31 {
    pub fn reset(&mut self){
        self.hand = vec![];
    }

    pub fn play_again(&mut self){
        self.hand.clear();
    }

    pub fn count_points_hand(&mut self, with_box: bool) {
        todo!() // TODO:
        
        // self.cur_score = self.hand.iter().map(|e| {
        //     match e % 13 {
        //         0 => 15,
        //         13|12|11|10 => 10,
        //         _ => 5
        //     }
        // }).sum::<u32>();
        // if with_box {
        //     self.cur_score = 25;
        // }
        // self.total_score += self.cur_score;
    }
}

impl From <&PlayerLobby> for Player31 {
    fn from(p: &PlayerLobby) -> Self {
        Player31 {
            id: p.id.to_owned(),
            name: p.name.to_owned(),
            is_bot: p.is_bot,
            hand: vec![],
            total_wins: 0,
        }
    }
}