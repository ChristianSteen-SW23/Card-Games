use crate::objects::traits::has_players::HasPlayers;
use crate::objects::turn_manager::TurnManager;
use crate::objects::{game7::player7::Player7, lobby::lobby::Lobby};
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

    // fn deal_cards(&mut self) {
    //     let mut card_deck: Vec<u32> = (0..=51).collect();
    //     let mut rng = rng();

    //     while !card_deck.is_empty() {
    //         let random_num = rng.random_range(0..card_deck.len() as u32) as usize;
    //         let cur_id = self.turn_manager.get_current();
    //         if card_deck[random_num] == 19 {
    //             self.starting_player_id = cur_id.to_string()
    //         }
    //         card_deck.remove(random_num);
    //         self.get_mut_player(cur_id)
    //             .expect("Not possible")
    //             .hand
    //             .push(card_deck[random_num]);
    //         let ids = self.player_ids().clone(); // compute before the mutable borrow needed by advance_turn, if that also takes &mut
    //         self.turn_manager.advance_turn(&ids);
    //     }
    // }
    fn deal_cards(&mut self) {
        let mut card_deck: Vec<u32> = (0..=51).collect();
        let mut rng = rng();

        while !card_deck.is_empty() {
            let random_num = rng.random_range(0..card_deck.len() as u32) as usize;
            print!("{}", random_num);
            let cur_id: String = self.turn_manager.get_current().to_string();
            if card_deck[random_num] == 19 {
                self.starting_player_id = cur_id.clone();
            }
            self.get_mut_player(&cur_id)
                .expect("Not possible")
                .hand
                .push(card_deck[random_num]); // Works with just "2"
            self.turn_manager.advance_turn(&self.player_ids());
            card_deck.remove(random_num);
        }
    }

    // pub fn handle_move(&mut self, data: Game7Payload, sid: &str) -> Result<(), Error> {
    //     match data.move_type {
    //         crate::socket::game_7_socket::Game7Events::PlayAgain => todo!(),
    //         crate::socket::game_7_socket::Game7Events::SkipTurn => {
    //             self.turn_manager.check_turn_with_error(sid)?;
    //             self.skip_turn(sid)
    //         }
    //         crate::socket::game_7_socket::Game7Events::PlayCard => {
    //             self.turn_manager.check_turn_with_error(sid)?;
    //             let card = data
    //                 .card
    //                 .ok_or_else(|| Error::Game7Error("Did not send any card".into()))?;
    //             self.play_card(card, sid)?;

    //             todo!()
    //         }
    //     }
    // }

    // pub fn check_and_calculate_win(&mut self) -> Result<Option<i32>, Error> {
    //     if !self
    //         .game_data
    //         .players
    //         .iter()
    //         .any(|p| p.get_7_game().map(|g| g.cards_left == 0).unwrap_or(false))
    //     {
    //         return Ok(None);
    //     }

    //     self.game_data.players.iter().for_each(|p| p.get_7_game()?.count_and_reset_hand(true));

    //     todo!()
    // }

    // fn play_card(&mut self, card: i32, sid: &str) -> Result<(), Error> {
    //     let board = &self.board;
    //     let Some(player_ref) = self.game_data.players.get_mut(sid) else {
    //         return Err(Error::Game7Error("Player not found".into()));
    //     };
    //     let PlayerGameData::Player7(player7) = &mut player_ref.game else {
    //         return Err(Error::Game7Error("Expected Game7 data for player".into()));
    //     };

    //     if !card_playable(&(card), board) {
    //         return Err(Error::Game7Error("You can not play that card".into()));
    //     }
    //     if !player7.hand.contains(&(card as u32)) {
    //         return Err(Error::Game7Error(
    //             "You do not have that card in your hand".into(),
    //         ));
    //     }
    //     let Some(pos) = player7.hand.iter().position(|c| *c == card as u32) else {
    //         return Err(Error::Game7Error("Failed to remove card".into()));
    //     };
    //     player7.hand.remove(pos);
    //     player7.cards_left -= 1;

    //     let suit = ((card - (card % 13)) / 13) as usize;
    //     let rank = card % 13 + 1;
    //     if rank == 7 {
    //         self.board[1][suit] = rank;
    //     } else if rank > 7 {
    //         self.board[0][suit] = rank;
    //     } else if rank < 7 {
    //         self.board[2][suit] = rank;
    //     }

    //     self.turn_manager
    //         .advance_turn(&self.game_data.players)
    //         .map_err(|_| Error::Game7Error("Could not advance turn".into()))?;

    //     Ok(())
    // }

    // fn skip_turn(&mut self, sid: &str) -> Result<(), Error> {
    //     let board = &self.board;

    //     let Some(player_ref) = self.game_data.players.get(sid) else {
    //         return Err(Error::Game7Error("Player not found".into()));
    //     };

    //     let PlayerGameData::Player7(player7) = &player_ref.game else {
    //         return Err(Error::Game7Error("Expected Game7 data for player".into()));
    //     };

    //     if possible_skip(&player7.hand, &board) {
    //         self.turn_manager.advance_turn(&self.game_data.players);
    //         Ok(())
    //     } else {
    //         Err(Error::Game7Error("You can not skip right now".into()))
    //     }
    // }
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
