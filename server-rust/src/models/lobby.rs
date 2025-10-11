use std::{collections::HashMap, hash::Hash};

use rand::distr::Map;

use crate::{
    models::GameLogic,
    socket::lobby_socket::{LobbyResponse, PlayerResponse},
};

use super::player::Player;

#[derive(Debug, Clone)]
pub struct Lobby {
    pub id: u32,
    pub host: String, // player id
    pub game_started: bool,
    pub players: HashMap<String, Player>,
    // pub players: Vec<Player>,
    pub game: Option<GameLogic>,
}

impl Lobby {
    pub fn new(id: u32, host: String) -> Self {
        Self {
            id,
            host,
            game_started: false,
            players: HashMap::new(),
            game: None,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.id.clone(), player);
    }

    pub fn start_game(&mut self) {
        self.game_started = true;
    }

    pub fn to_response(&self) -> LobbyResponse {
        let players = self
            .players
            .values()
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
}
