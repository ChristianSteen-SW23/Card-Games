use crate::objects::{game7::Game7, lobby::lobby::Lobby};


#[derive(Debug, Clone)]
pub enum GameLogic {
    Game7(Game7),
    Lobby(Lobby),
    // Player31(Player31Data),
    // Player500(Player500Data),
    // PlanningPoker(PlayerPlanningPokerData),
}


// impl GameLogic {
//     pub fn get_players(&self) -> &Players {
//         match self {
//             GameLogic::Game7(game) => /*game.get_players()*/ todo!(),
//             GameLogic::Lobby(lobby) => lobby.get_players(),
//             //_ => vec![],
//         }
//     }
// }


