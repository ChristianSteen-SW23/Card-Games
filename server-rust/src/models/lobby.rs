

use socketioxide::{ SocketIo};

use crate::{
    models::{Game7Logic, GameLogic, Players},
    socket::lobby_socket::{LobbyResponse, PlayerResponse},
};

use super::player::Player;

#[derive(Debug, Clone)]
pub struct Lobby {
    pub id: u32,
    pub host: String, // player id
    pub game_started: bool,
    pub players: Players,
    pub game: Option<GameLogic>,
}

impl Lobby {
    pub fn new(id: u32, host: String) -> Self {
        Self {
            id,
            host,
            game_started: false,
            players: Players::new(),
            game: None,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.add(player);
    }

    pub fn start_game(&mut self) {
        self.game_started = true;
    }

    pub fn to_response(&self) -> LobbyResponse {
        let players = self
            .players
            .get_all()
            .iter()
            .map(|p| PlayerResponse {
                playerid: p.id.clone(),
                name: p.name.clone(),
                host: p.id == self.host,
            })
            .collect();

        LobbyResponse {
            id: self.id,
            players,
        }
    }

    pub fn play_again(&mut self, s_id: String, io: SocketIo) {
        match self.game {
            Some(GameLogic::Game7Logic(_)) => Game7Logic::play_again(self, s_id, io),
            None => todo!(),
        }
    }
}
