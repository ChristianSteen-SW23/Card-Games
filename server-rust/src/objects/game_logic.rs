use crate::objects::{LobbyLogic, Players};


#[derive(Debug, Clone)]
pub enum GameLogic {
    //Game7Logic(Game7Logic),
    LobbyLogic(LobbyLogic),
    // Player31(Player31Data),
    // Player500(Player500Data),
    // PlanningPoker(PlayerPlanningPokerData),
}


impl GameLogic {
    pub fn get_players(&self) -> &Players {
        match self {
            //GameLogic::Game7Logic(game) => game.get_players(),
            GameLogic::LobbyLogic(lobby) => lobby.get_players(),
            //_ => vec![],
        }
    }
}