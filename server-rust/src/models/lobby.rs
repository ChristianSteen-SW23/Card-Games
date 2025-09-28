use super::player::Player;

#[derive(Debug)]
pub struct Lobby {
    pub id: u32,
    pub host: String, // player id
    pub game_started: bool,
    pub players: Vec<Player>,
}

impl Lobby {
    pub fn new(id: u32, host: String) -> Self {
        Self {
            id,
            host,
            game_started: false,
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn start_game(&mut self) {
        self.game_started = true;
    }
}
