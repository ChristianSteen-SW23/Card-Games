use rand::{Rng, rng};
use socketioxide::extract::SocketRef;

use crate::models::{Lobby, Player7Data, PlayerGameData, TurnManager};

#[derive(Debug, Clone)]
pub struct Game7Logic {
    pub board: Vec<Vec<Option<u32>>>,
    pub r#box: Option<String>,
    pub turn_manager: TurnManager,
    pub starting_player_id: String,
}

impl Game7Logic {
    pub fn new(lobby: &mut Lobby, s: SocketRef) -> Self {
        let mut game = Self {
            board: vec![],
            turn_manager: TurnManager::new(),
            r#box: None,
            starting_player_id: "".to_string(),
        };

        game.start_game(lobby, s);

        game
    }

    fn start_game(&mut self, lobby: &mut Lobby, s: SocketRef) {
        println!("Starting Game 7 with {} players", lobby.players.len());

        for player in &mut lobby.players {
            player.game = Some(PlayerGameData::Player7(Player7Data::new()));
        }
        self.turn_manager.set_current(s.id.to_string());
        start_game_helper(self, lobby);

        self.turn_manager
            .set_current(self.starting_player_id.clone());
        self.turn_manager.set_next(&lobby.players);
        println!("{:?}", lobby);
    }

    fn handle_move(&mut self, player_id: &str, data: serde_json::Value) {
        println!("Player {} made a move: {:?}", player_id, data);
    }
}

fn start_game_helper(data: &mut Game7Logic, lobby: &mut Lobby) {
    data.r#box = None;
    let rows = 3;
    let cols = 4;
    data.board = vec![vec![None; cols]; rows];

    data.turn_manager.advance_turn(&(lobby.players));
    deal_cards(data, lobby);
    data.turn_manager
        .set_current(data.starting_player_id.clone());
    data.turn_manager.set_next(&(lobby.players));

    for player in lobby.clone().players.into_iter() {
        match player.game.unwrap() {
            PlayerGameData::Player7(mut player) => player.set_cards_left(),
        }
    }
}

fn deal_cards(data: &mut Game7Logic, lobby: &mut Lobby) {
    let mut card_deck: Vec<u32> = (1..52).collect();

    let mut rng = rng();
    let mut random_num: u32;
    let mut i = 0;
    while card_deck.len() != 0 {
        random_num = {
            let this = &mut rng;
            let range = 0..(card_deck.len() as u32);
            this.random_range(range)
        };
        println!("{:?}", random_num);

        if card_deck[random_num as usize] == 19 {
            data.starting_player_id = data.turn_manager.get_current().unwrap().to_string();
        }
        if let Some(game) = &mut lobby.players[i].game {
            match game {
                PlayerGameData::Player7(player) => {
                    player.hand.push(card_deck[random_num as usize]);
                }
            }
        }
        card_deck.remove(random_num as usize);
        i = (i + 1) % lobby.players.len();
    }
}
