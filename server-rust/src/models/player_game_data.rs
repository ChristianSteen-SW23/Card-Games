use crate::models::Player7Data;

#[derive(Debug, Clone)]
pub enum PlayerGameData {
    Player7(Player7Data),
    // Player31(Player31Data),
    // Player500(Player500Data),
    // PlanningPoker(PlayerPlanningPokerData),
}