use crate::models::Player;

#[derive(Debug, Clone)]
pub struct TurnManager{
    current: String,
    next: String,
}

impl TurnManager {
    fn set_current (&mut self, player_id: String){
        todo![]
    }

    fn get_current (&self){
        todo![]
    }

    fn advance_turn (&mut self, players: &[Player]){
        todo![]
    }

}