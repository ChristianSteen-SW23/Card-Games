use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::models::{Lobby, Player};

#[derive(Debug, Default, Clone)]
pub struct ServerState {
    // all lobbies by lobby id
    pub lobbies: HashMap<u32, Arc<Lobby>>,
    // player_id -> lobby_id
    pub player_lobby: HashMap<String, u32>,
}
