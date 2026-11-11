use crate::objects::{game7::Game7, game31::Game31, lobby::lobby::Lobby, planning_poker::game_planning_poker::PlanningPoker};


#[derive(Debug, Clone)]
pub enum GameLogic {
    Game7(Game7),
    Lobby(Lobby),
    Game31(Game31),
    PlanningPoker(PlanningPoker),
}

