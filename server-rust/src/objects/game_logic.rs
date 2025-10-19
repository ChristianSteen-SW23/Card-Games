use crate::objects::{Game7Logic, LobbyLogic};


#[derive(Debug, Clone)]
pub enum GameLogic {
    //Game7Logic(Game7Logic),
    LobbyLogic(LobbyLogic),
    // Player31(Player31Data),
    // Player500(Player500Data),
    // PlanningPoker(PlayerPlanningPokerData),
}

