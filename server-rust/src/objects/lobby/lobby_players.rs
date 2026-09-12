


use std::{slice::Iter};

use crate::objects::lobby::lobby_player::PlayerLobby;

#[derive(Debug, Clone)]
pub struct PlayersLobby(Vec<PlayerLobby>);

impl PlayersLobby {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn get(&self, key: &str) -> Option<&PlayerLobby> {
        self.0.iter().find(|player| player.id == key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut PlayerLobby> {
        self.0.iter_mut().find(|player| player.id == key)
    }

    pub fn get_all(&self) -> &Vec<PlayerLobby> {
        &self.0
    }

    pub fn get_next(&self, key: &str) -> Option<&PlayerLobby> {
        self.0
            .iter()
            .position(|player| player.id == key)
            .and_then(|idx| self.0.get((idx + 1) % self.0.len()))
    }

    pub fn get_mut_all(&mut self) -> &mut Vec<PlayerLobby> {
        &mut self.0
    }

    pub fn add(&mut self, player: PlayerLobby) {
        self.0.push(player);
    }

    pub fn remove(&mut self, key: &str) {
        let Some(idx) = self.0.iter().position(|player| player.id == key) else {
            return;
        };
        self.0.remove(idx);
    }

    pub fn iter(&self) -> Iter<'_, PlayerLobby> {
        self.0.iter()
    }
}

impl From<PlayerLobby> for PlayersLobby {
    fn from(value: PlayerLobby) -> Self {
        Self(vec![value])
    }
}
