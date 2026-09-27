use crate::objects::traits::has_players::HasPlayers;
use crate::objects::turn_manager::TurnManager;
use crate::objects::{game7::player7::Player7, lobby::lobby::Lobby};
use crate::socket::Game7Payload;
use crate::socket::send_error_socket::Error;
use rand::{Rng, rng};
use serde::de::value;
use std::fmt::format;

#[derive(Debug, Clone)]
pub struct Game7 {
    pub id: u32,
    pub host: String,
    pub players: Vec<Player7>,
    pub board: Vec<Vec<i32>>,
    pub r#box: Option<String>,
    pub turn_manager: TurnManager,
    pub starting_player_id: String,
}

impl From<Lobby> for Game7 {
    fn from(value: Lobby) -> Game7 {
        let new_players: Vec<Player7> = value.players.get_all().iter().map(Player7::from).collect();
        let mut game = Game7 {
            id: value.id,
            host: value.host,
            players: new_players,
            board: vec![vec![0; 4]; 3],
            r#box: None,
            starting_player_id: String::from(""),
            turn_manager: TurnManager::new(),
        };
        game.set_up_game();
        game
    }
}
// Impls used on players
impl Game7 {
    pub fn get_player(&self, key: &str) -> Option<&Player7> {
        self.players.iter().find(|p| p.id == key)
    }

    pub fn get_mut_player(&mut self, key: &str) -> Option<&mut Player7> {
        self.players.iter_mut().find(|player| player.id == key)
    }

    pub fn get_mut_player_result(&mut self, key: &str) -> Result<&mut Player7, Error> {
        match self.players.iter_mut().find(|player| player.id == key) {
            Some(p) => Ok(p),
            None => Err(Error::Game7Error(format!(
                "Could not find player ID {}",
                key
            ))),
        }
    }
}
impl Game7 {
    pub fn set_up_game(&mut self) {
        self.board = vec![vec![0; 4]; 3];
        self.r#box = None;
        self.starting_player_id = String::new();

        self.players.iter_mut().for_each(|player| player.reset());

        self.start_game();
    }

    fn start_game(&mut self) {
        println!("Starting Game 7 with {} players", self.players.len());

        let _ = self
            .turn_manager
            .update(self.host.to_string(), &self.player_ids());
        self.deal_cards();
        self.players.iter_mut().for_each(|p| {
            p.cards_left = p.hand.len();
        });

        let _ = self
            .turn_manager
            .update(self.starting_player_id.to_owned(), &self.player_ids());
    }

    pub fn handle_move(&mut self, data: Game7Payload, sid: &String) -> Result<(), Error> {
        match data.move_type {
            crate::socket::game_7_socket::Game7Events::PlayCard => {
                self.turn_manager.check_turn_with_error(sid)?;
                self.play_card(
                    data.card
                        .ok_or_else(|| Error::Game7Error("Did not send any card".into()))?,
                    sid,
                )
            }
            crate::socket::game_7_socket::Game7Events::SkipTurn => {
                self.turn_manager.check_turn_with_error(sid)?;
                self.skip_turn(sid)
            }
            crate::socket::game_7_socket::Game7Events::PlayAgain => {self.set_up_game(); Ok(())},
        }
    }

    fn skip_turn(&mut self, sid: &String) -> Result<(), Error> {
        let board = &self.board;

        let Some(player_ref) = self.get_player(sid) else {
            return Err(Error::Game7Error("Player not found".into()));
        };

        if possible_skip(&player_ref.hand, &board) {
            let _ = self.turn_manager.advance_turn(&self.player_ids());
            self.r#box = Some(sid.to_string());
            Ok(())
        } else {
            Err(Error::Game7Error("You can not skip right now... 😿".into()))
        }
    }

    fn play_card(&mut self, card_num: i32, sid: &String) -> Result<(), Error> {
        let board = &self.board;
        if !card_playable(&(card_num), board) {
            return Err(Error::Game7Error("You can not play that card".into()));
        }

        let Some(player_ref) = self.get_mut_player(sid) else {
            return Err(Error::Game7Error("Player not found".into()));
        };

        if !player_ref.hand.contains(&(card_num as u32)) {
            return Err(Error::Game7Error(
                "You do not have that card in your hand".into(),
            ));
        }
        let Some(pos) = player_ref.hand.iter().position(|c| *c == card_num as u32) else {
            return Err(Error::Game7Error("Failed to remove card".into()));
        };
        player_ref.hand.remove(pos);
        player_ref.cards_left -= 1;

        let suit = ((card_num - (card_num % 13)) / 13) as usize;
        let rank = card_num % 13 + 1;
        if rank == 7 {
            self.board[1][suit] = rank;
        } else if rank > 7 {
            self.board[0][suit] = rank;
        } else if rank < 7 {
            self.board[2][suit] = rank;
        }

        self.turn_manager
            .advance_turn(&self.player_ids())
            .map_err(|_| Error::Game7Error("Could not advance turn".into()))?;

        Ok(())
    }

    fn deal_cards(&mut self) {
        let mut card_deck: Vec<u32> = (0..=51).collect();
        let mut rng = rng();

        while !card_deck.is_empty() {
            let random_num = rng.random_range(0..card_deck.len() as u32) as usize;
            let cur_id: String = self.turn_manager.get_current().to_string();
            if card_deck[random_num] == 19 {
                self.starting_player_id = cur_id.clone();
            }
            self.get_mut_player(&cur_id)
                .expect("Not possible")
                .hand
                .push(card_deck[random_num]); // Works with just "2"
            let _ = self.turn_manager.advance_turn(&self.player_ids());
            card_deck.remove(random_num);
        }
    }

    pub fn check_and_calculate_win(&mut self) -> Result<Option<()>, Error> {
        if !self.players.iter().any(|p| p.cards_left == 0) {
            return Ok(None);
        }

        let box_id = self.r#box.clone().unwrap_or_default();
        self.players.iter_mut().for_each(|p| {
            if p.id == box_id {
                p.count_and_reset_hand(true)
            } else {
                p.count_and_reset_hand(false)
            }
        });
        Ok(Some(()))
    }
}

fn card_playable(card: &i32, board: &Vec<Vec<i32>>) -> bool {
    let suit = ((card - (card % 13)) / 13) as usize;
    let rank = card % 13 + 1;
    // println!("{}, {}, {}",card, suit,rank);

    if board[1][1] == 0 && *card != 19 {
        return false;
    }
    if rank == 7 {
        return true;
    }
    if board[1][suit] == 0 {
        return false;
    }
    if rank == 6 || rank == 8 {
        return true;
    }
    if rank > 7 && board[0][suit] + 1 == rank {
        return true;
    }
    if rank < 7 && board[2][suit] - 1 == rank {
        return true;
    }
    false
}

fn possible_skip(hand: &Vec<u32>, board: &Vec<Vec<i32>>) -> bool {
    !hand
        .iter()
        .any(|card| card_playable(&(*card as i32), board))
}
