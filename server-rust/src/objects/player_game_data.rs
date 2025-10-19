use crate::objects::Player7Data;

#[derive(Debug, Clone)]
pub enum PlayerGameData {
    Player7(Player7Data),
    Player31(u32),
    Lobby
    // Player500(Player500Data),
    // PlanningPoker(PlayerPlanningPokerData),
}