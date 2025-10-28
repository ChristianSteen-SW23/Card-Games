use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::objects::{GameLogic, LobbyLogic};
use crate::socket::send_error_socket::Error;

pub type SharedState = Arc<Mutex<ServerState>>;

#[derive(Debug, Default, Clone)]
pub struct ServerState {
    // all lobbies by lobby id
    pub game_map: HashMap<u32, Arc<Mutex<GameLogic>>>,
    // player_id -> lobby_id
    pub player_lobby_map: HashMap<String, u32>,
}

// impl ServerState {
//     pub fn add_player_lobby(&mut self, lobby_id: u32, socket_id: String) {
//         self.player_lobby.insert(socket_id, lobby_id);
//     }
// }

impl ServerState {
    pub fn new() -> Self {
        Self {
            game_map: HashMap::new(),
            player_lobby_map: HashMap::new(),
        }
    }

    pub fn insert_game(&mut self, lobby: LobbyLogic, lobby_id: u32) {
        self.game_map.insert(lobby_id, Arc::new(Mutex::new(GameLogic::LobbyLogic(lobby))));
    }

    pub fn insert_player_lobby(&mut self, sid: String, lobby_id: u32) {
        self.player_lobby_map.insert(sid, lobby_id);
    }
    
    // pub fn add_player_to_lobby(&mut self, lobby_id: u32, player: Player) {
    //     if let Some(lobby_arc) = self.games.get(&lobby_id) {
    //         self.player_lobby.insert(player.id.clone(), lobby_id);
    //         if let Ok(mut lobby) = lobby_arc.lock() {
    //             lobby.add_player(player);
    //         }
    //     }
    // }

    pub fn delete_room(&mut self, lobby_id: &u32) {
        self.game_map
            .get(lobby_id)
            .unwrap()
            .lock()
            .unwrap().get_players().get_all().iter()
            .for_each(|player| {
                self.player_lobby_map.remove(&player.id);
            });
        self.game_map.remove(lobby_id);
    }

    // pub fn delete_player(&mut self, lobby_id: &u32, socket_id: String) {
    //     self.player_lobby.remove(&socket_id);

    //     if let Some(lobby_arc) = self.games.get(lobby_id) {
    //         let mut lobby = lobby_arc.lock().unwrap();
    //         lobby.players.remove(&socket_id);
    //     }
    // }

    pub fn get_lobby_mut<'a>(
        &'a mut self,
        socket_id: &str,
    ) -> Result<MutexGuard<'a, GameLogic>, Error> {
        let lobby_id = self
            .player_lobby_map
            .get(socket_id)
            .ok_or_else(|| Error::LobbyError("Player not in any lobby".to_string()))?
            .clone();

        let lobby_arc = self
            .game_map
            .get(&lobby_id)
            .ok_or_else(|| Error::LobbyError("Lobby not found".to_string()))?;

        Ok(lobby_arc.lock().unwrap())
    }

    pub fn get_lobby_arc(&self, sid: &str) -> Result<Arc<Mutex<GameLogic>>, Error> {
        let lobby_id = self.player_lobby_map
            .get(sid)
            .ok_or(Error::LobbyError("Player not in a lobby".into()))?;

        let lobby_arc = self.game_map
            .get(lobby_id)
            .ok_or(Error::LobbyError("Lobby not found".into()))?;

        Ok(lobby_arc.clone())
    }
}

