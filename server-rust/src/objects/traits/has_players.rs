use crate::objects::{GameLogic, game7::Game7, lobby::lobby::Lobby};

pub(crate) trait HasPlayers {
    fn player_ids(&self) -> Vec<String>;
}

impl HasPlayers for Lobby {
    fn player_ids(&self) -> Vec<String> {
        self.players.iter().map(|p| p.id.clone()).collect()
    }
}

impl HasPlayers for Game7 {
    fn player_ids(&self) -> Vec<String> {
        self.players.iter().map(|p| p.id.clone()).collect()
    }
}


impl HasPlayers for GameLogic {
    fn player_ids(&self) -> Vec<String> {
        match self {
            GameLogic::Lobby(l) => l.player_ids(),
            GameLogic::Game7(g) => g.player_ids(),
            // GameLogic::Game31Logic(g) => g.player_ids(),
            // GameLogic::Game500Logic(g) => g.player_ids(),
            // GameLogic::PlanningPokerLogic(g) => g.player_ids(),
        }
    }
}