use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::objects::{GameData, Player};

pub type SharedState = Arc<Mutex<ServerState>>;

#[derive(Debug, Default, Clone)]
pub struct ServerState {
    // all lobbies by lobby id
    pub games: HashMap<u32, Arc<Mutex<GameData>>>,
    // player_id -> lobby_id
    pub player_lobby: HashMap<String, u32>,
}

// impl ServerState {
//     pub fn add_player_lobby(&mut self, lobby_id: u32, socket_id: String) {
//         self.player_lobby.insert(socket_id, lobby_id);
//     }
// }

impl ServerState {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
            player_lobby: HashMap::new(),
        }
    }

    // pub fn add_lobby(&mut self, lobby: GameData) {
    //     self.games.insert(lobby.id, Arc::new(Mutex::new(lobby)));
    // }

    // pub fn add_player_to_lobby(&mut self, lobby_id: u32, player: Player) {
    //     if let Some(lobby_arc) = self.games.get(&lobby_id) {
    //         self.player_lobby.insert(player.id.clone(), lobby_id);
    //         if let Ok(mut lobby) = lobby_arc.lock() {
    //             lobby.add_player(player);
    //         }
    //     }
    // }

    // pub fn delete_room(&mut self, lobby_id: &u32) {
    //     self.games
    //         .get(lobby_id)
    //         .unwrap()
    //         .lock()
    //         .unwrap()
    //         .players.get_all().iter()
    //         .for_each(|player| {
    //             self.player_lobby.remove(&player.id);
    //         });
    //     self.games.remove(lobby_id);
    // }

    // pub fn delete_player(&mut self, lobby_id: &u32, socket_id: String) {
    //     self.player_lobby.remove(&socket_id);

    //     if let Some(lobby_arc) = self.games.get(lobby_id) {
    //         let mut lobby = lobby_arc.lock().unwrap();
    //         lobby.players.remove(&socket_id);
    //     }
    // }
}

