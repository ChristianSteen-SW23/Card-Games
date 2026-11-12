use std::usize;

use crate::objects::lobby::lobby::Lobby;
use crate::objects::traits::has_players::HasPlayers;
use crate::objects::turn_manager::TurnManager;
use crate::socket::game_31_socket::Game31Events;
use crate::socket::send_error_socket::Error;
use crate::{objects::game31::player31::Player31, socket::Game31Payload};
use rand::seq::SliceRandom;
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
    game_over: bool,
    first_turn: bool,
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
            game_over: false,
            first_turn: true,
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
    pub fn get_game_over(&self) -> bool {
        self.game_over
    }
}

impl Game31 {
    pub fn set_game_over(&mut self) -> bool {
        match self.game_over {
            true => {
                self.players.iter_mut().for_each(|p| {
                    p.count_points_hand();
                    
                });
                let max_score = self.players.iter().map(|p| p.cur_score).max().unwrap_or(0);

                self.players
                    .iter_mut()
                    .filter(|p| p.cur_score == max_score)
                    .for_each(|p| p.total_wins += 1);

                true
            }
            false => false,
        }
    }

    pub fn set_up_game(&mut self) {
        self.starting_player_id = String::new();

        self.players.iter_mut().for_each(|player| player.reset());
        self.card_pile = (0..51).collect();
        self.game_over = false;
        self.first_turn = true;

        self.start_game();
    }

    fn start_game(&mut self) {
        println!("Starting Game 31 with {} players", self.players.len());
        let _ = self
            .turn_manager
            .update(self.host.to_string(), &self.player_ids());
        self.deal_cards();
        self.starting_player_id = self.turn_manager.get_current().into();

        let mut rng = rand::rng();
        self.card_pile.shuffle(&mut rng);
    }

    fn deal_cards(&mut self) {
        let mut rng = rng();
        // Takes a card to be the top of the stack on the table
        let random_num = rng.random_range(0..self.card_pile.len() as u32) as usize;
        self.top_stack_card = self.card_pile.remove(random_num);

        for _ in 1..=(3 * self.get_players().len()) {
            let random_num = rng.random_range(0..self.card_pile.len() as u32) as usize;
            let cur_id: String = self.turn_manager.get_current().to_string();
            let card = self.card_pile.remove(random_num);
            self.get_mut_player(&cur_id)
                .expect("Not possible")
                .hand
                .push(card);
            let _ = self.turn_manager.advance_turn(&self.player_ids());
        }

        self.players.iter_mut().for_each(|p| p.hand.sort());
    }

    pub fn handle_move(&mut self, data: Game31Payload, sid: &String) -> Result<(), Error> {
        let is_new_game = matches!(data.move_type, Game31Events::NewGame);
        if self.game_over != is_new_game {
            return Err(Error::Game31Error(if self.game_over {
                "Game is over, click play again".into()
            } else {
                "The game is not over yet".into()
            }));
        }

        self.confirm_turn(sid)?;
        match data.move_type {
            Game31Events::Swap => self.swap_card(data.card, sid),
            Game31Events::PickTopCard => self.pick_top_card(sid),
            Game31Events::PlayTopCard => self.play_top_card(data.card, sid),
            Game31Events::Knock => self.knock_table(sid),
            Game31Events::NewGame => {
                self.set_up_game();
                Ok(())
            }
        }?;

        match data.move_type {
            Game31Events::Swap | Game31Events::PlayTopCard | Game31Events::Knock => {
                let turn_player = self
                    .get_mut_player(sid)
                    .ok_or_else(|| {
                        Error::Game31Error("Could not find your player ID in the game data".into())
                    })?
                    .count_points_hand();
                self.game_over |= turn_player.cur_score == 31;

                self.turn_manager
                    .advance_turn(&self.player_ids())
                    .map_err(|_| Error::Game31Error("Could not advance turn".into()))?;

                let cur_player = self.turn_manager.get_current().to_string();
                self.first_turn &= cur_player != self.starting_player_id;
                self.game_over |= self
                    .get_mut_player(&cur_player)
                    .ok_or_else(|| {
                        Error::Game31Error("Could not find your player ID in the game data".into())
                    })?
                    .has_knocked;

                Ok(())
            }
            Game31Events::PickTopCard | Game31Events::NewGame => Ok(()),
        }
    }
    fn knock_table(&mut self, sid: &String) -> Result<(), Error> {
        if self.first_turn {
            return Err(Error::Game31Error("You can not knock first turn".into()));
        }

        if self
            .get_mut_player(sid)
            .ok_or_else(|| {
                Error::Game31Error("Could not find your player ID in the game data".into())
            })?
            .hand
            .len()
            >= 4
        {
            return Err(Error::Game31Error(
                "You can not pick a card and knock the table, you have to play a card first".into(),
            ));
        }
        self.get_mut_player(sid)
            .ok_or_else(|| {
                Error::Game31Error("Could not find your player ID in the game data".into())
            })?
            .has_knocked = true;

        Ok(())
    }

    fn confirm_turn(&mut self, sid: &String) -> Result<(), Error> {
        if self.game_over {
            Ok(())
        } else {
            self.turn_manager.check_turn_with_error(sid)
        }
    }

    fn swap_card(&mut self, card_idx: Option<u32>, sid: &String) -> Result<(), Error> {
        let Some(card_idx) = card_idx.map(|c| c as usize) else {
            return Err(Error::Game31Error(
                "You did not send a card in your request".into(),
            ));
        };

        if !(card_idx < 3) {
            return Err(Error::Game31Error(
                "You have not picked a card in your hand to swap".into(),
            ));
        }
        let top_card = self.top_stack_card;

        let Some(player) = self.get_mut_player(sid) else {
            return Err(Error::Game31Error(
                "Could not find your player ID in the game data".into(),
            ));
        };
        let old_card = player.hand[card_idx];
        player.hand[card_idx] = top_card;
        player.hand.sort();
        self.top_stack_card = old_card;

        Ok(())
    }

    fn pick_top_card(&mut self, sid: &String) -> Result<(), Error> {
        if self
            .get_mut_player(sid)
            .ok_or_else(|| {
                Error::Game31Error("Could not find your player ID in the game data".into())
            })?
            .hand
            .len()
            >= 4
        {
            return Err(Error::Game31Error(
                "You have picked one card all ready".into(),
            ));
        }
        let top_card = self
            .card_pile
            .pop()
            .ok_or_else(|| Error::Game31Error(String::from("The card pile is empty")))?;
        self.get_mut_player(sid)
            .ok_or_else(|| {
                Error::Game31Error("Could not find your player ID in the game data".into())
            })?
            .hand
            .push(top_card);
        Ok(())
    }

    fn play_top_card(&mut self, card_idx: Option<u32>, sid: &String) -> Result<(), Error> {
        let Some(card_idx) = card_idx.map(|c| c as usize) else {
            return Err(Error::Game31Error(
                "You did not send a card in your request".into(),
            ));
        };

        if self
            .get_mut_player(sid)
            .ok_or_else(|| {
                Error::Game31Error("Could not find your player ID in the game data".into())
            })?
            .hand
            .len()
            != 4
        {
            return Err(Error::Game31Error(
                "You can not play a card without taking one from the pile first".into(),
            ));
        }

        if !(card_idx < 4) {
            return Err(Error::Game31Error(
                "You have not picked a card in your hand to swap".into(),
            ));
        }

        self.top_stack_card = self
            .get_mut_player(sid)
            .ok_or_else(|| {
                Error::Game31Error("Could not find your player ID in the game data".into())
            })?
            .hand
            .remove(card_idx);

        Ok(())
    }
}
