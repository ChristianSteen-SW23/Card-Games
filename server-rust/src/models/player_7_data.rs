
#[derive(Debug, Clone)]
pub struct Player7Data {
    pub hand: Vec<u32>,
    pub total_score: u32,
    pub cards_left: usize,
}

impl Player7Data {
    pub fn new() -> Self {
        Self {
            hand: vec![],
            total_score: 0,
            cards_left: 0,
        }
    }

    pub fn set_cards_left(&mut self){
        self.cards_left = self.hand.len();
    }

    pub fn play_again(&mut self){
        self.cards_left = 0;
        self.hand.clear();
    }
}

