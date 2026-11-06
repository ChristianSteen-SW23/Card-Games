use crate::socket::game_31_socket::Game31Events;
use crate::{objects::game31::player31::Player31, socket::Game31Payload};
use crate::objects::lobby::lobby::Lobby;
use crate::objects::traits::has_players::HasPlayers;
use crate::objects::turn_manager::TurnManager;
use crate::socket::Game7Payload;
use crate::socket::send_error_socket::Error;
use rand::{Rng, rng};

#[derive(Debug, Clone)]
pub struct Game31 {
    id: u32,
    host: String,
    players: Vec<Player31>,
    turn_manager: TurnManager,
    starting_player_id: String,
    card_pile: Vec<u32>,
    top_stack_card: u32,
}

impl From<Lobby> for Game31 {
    fn from(value: Lobby) -> Game31 {
        let new_players: Vec<Player31> =
            value.players.get_all().iter().map(Player31::from).collect();
        let mut game = Game31 {
            id: value.id,
            host: value.host,
            players: new_players,
            starting_player_id: String::from(""),
            turn_manager: TurnManager::new(),
            card_pile: (0..=51).collect(),
            top_stack_card: 0,
        };
        game.start_game();
        game
    }
}
// Impls used on players
impl Game31 {
    pub fn get_player(&self, key: &str) -> Option<&Player31> {
        self.players.iter().find(|p| p.id == key)
    }

    pub fn get_mut_player(&mut self, key: &str) -> Option<&mut Player31> {
        self.players.iter_mut().find(|player| player.id == key)
    }

    pub fn get_mut_player_result(&mut self, key: &str) -> Result<&mut Player31, Error> {
        match self.players.iter_mut().find(|player| player.id == key) {
            Some(p) => Ok(p),
            None => Err(Error::Game31Error(format!(
                "Could not find player ID {}",
                key
            ))),
        }
    }
}

// Getters and Setters
impl Game31 {
    pub fn get_players(&self) -> &Vec<Player31> {
        &self.players
    }
    pub fn get_host(&self) -> &String {
        &self.host
    }
    pub fn get_top_stack_card(&self) -> u32 {
        self.top_stack_card
    }
    pub fn get_turn_manager(&self) -> &TurnManager {
        &self.turn_manager
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl Game31 {
    pub fn set_up_game(&mut self) {
        self.starting_player_id = String::new();

        self.players.iter_mut().for_each(|player| player.reset());
        self.card_pile = (0..51).collect();

        self.start_game();
    }

    fn start_game(&mut self) {
        println!("Starting Game 31 with {} players", self.players.len());
        let _ = self
            .turn_manager
            .update(self.host.to_string(), &self.player_ids());
        self.deal_cards();
    }

    fn deal_cards(&mut self) {
        let mut rng = rng();
        // Takes a card to be the top of the stack on the table
        let random_num = rng.random_range(0..self.card_pile.len() as u32) as usize;
        self.top_stack_card = self.card_pile.remove(random_num);

        for _ in 1..=(3*self.get_players().len()) {
            let random_num = rng.random_range(0..self.card_pile.len() as u32) as usize;
            let cur_id: String = self.turn_manager.get_current().to_string();
            let card = self.card_pile.remove(random_num);
            self.get_mut_player(&cur_id)
                .expect("Not possible")
                .hand
                .push(card);
            let _ = self.turn_manager.advance_turn(&self.player_ids());
        }
    }

    pub fn handle_move(&mut self, data: Game31Payload, sid: &String) -> Result<(), Error> {
        match data.move_type {
            Game31Events::Swap => self.swap_card(data.card, sid),
            Game31Events::PickTopCard => self.pick_top_card(sid),
            Game31Events::PlayTopCard => todo!(),
        }
    }

    fn swap_card(&mut self, card_idx: Option<u32>, sid: &String) -> Result<(), Error>{
        todo!()
    }

    fn pick_top_card(&mut self, sid: &String) -> Result<(), Error>{
        todo!()
    }
}
