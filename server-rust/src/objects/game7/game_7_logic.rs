use rand::{Rng, rng};

use crate::{
    objects::{GameData, LobbyLogic, Player7Data, PlayerGameData, TurnManager, turn_manager},
    socket::{Game7Payload, send_error_socket::Error},
};

#[derive(Debug, Clone)]
pub struct Game7Logic {
    pub board: Vec<Vec<i32>>,
    pub r#box: Option<String>,
    pub turn_manager: TurnManager,
    pub starting_player_id: String,
    pub game_data: GameData,
}
impl Game7Logic {
    pub fn new_from_lobby(lobby: LobbyLogic) -> Self {
        let mut lobby = Self {
            game_data: lobby.game_data,
            board: vec![vec![0; 4]; 3],
            r#box: None,
            turn_manager: TurnManager::new(),
            starting_player_id: String::new(),
        };

        lobby
            .game_data
            .players
            .get_mut_all()
            .iter_mut()
            .for_each(|player| player.game = PlayerGameData::Player7(Player7Data::new()));

        lobby.start_game();

        lobby
    }

    pub fn play_again(&mut self) {
        self.board = vec![vec![0; 4]; 3];
        self.r#box = None;
        self.starting_player_id = String::new();

        self.game_data
            .players
            .get_mut_all()
            .iter_mut()
            .for_each(|player| player.reset());

        self.start_game();
    }

    fn start_game(&mut self) {
        println!(
            "Starting Game 7 with {} players",
            self.game_data.players.get_all().len()
        );

        self.turn_manager
            .update(self.game_data.host.to_string(), &self.game_data.players);
        self.deal_cards();
        self.game_data
            .players
            .get_mut_all()
            .iter_mut()
            .for_each(|p| {
                if let PlayerGameData::Player7(d) = &mut p.game {
                    d.cards_left = d.hand.len();
                }
            });

        self.turn_manager
            .update(self.starting_player_id.to_owned(), &self.game_data.players);
    }

    fn deal_cards(&mut self) {
        let mut card_deck: Vec<u32> = (0..=51).collect();
        let mut rng = rng();

        while !card_deck.is_empty() {
            let random_num = rng.random_range(0..card_deck.len() as u32) as usize;
            let cur_id = self.turn_manager.get_current();
            if card_deck[random_num] == 19 {
                self.starting_player_id = cur_id.to_string()
            }
            match &mut self.game_data.players.get_mut(cur_id).unwrap().game {
                PlayerGameData::Player7(player7_data) => {
                    player7_data.hand.push(card_deck[random_num])
                }
                _ => unreachable!(),
            }
            card_deck.remove(random_num);
            self.turn_manager.advance_turn(&self.game_data.players);
        }
    }

    pub fn handle_move(&mut self, data: Game7Payload, sid: &str) -> Result<(), Error> {
        match data.move_type {
            crate::socket::game_7_socket::Game7Events::PlayAgain => todo!(),
            crate::socket::game_7_socket::Game7Events::SkipTurn => {
                self.turn_manager.check_turn_with_error(sid)?;
                self.skip_turn(sid)
            }
            crate::socket::game_7_socket::Game7Events::PlayCard => {
                self.turn_manager.check_turn_with_error(sid)?;
                let card = data
                    .card
                    .ok_or_else(|| Error::Game7Error("Did not send any card".into()))?;
                self.play_card(card, sid)
            }
        }
    }

    fn play_card(&mut self, card: i32, sid: &str) -> Result<(), Error> {
        let board = &self.board;
        let Some(player_ref) = self.game_data.players.get(sid) else {
            return Err(Error::Game7Error("Player not found".into()));
        };
        let PlayerGameData::Player7(player7) = &player_ref.game else {
            return Err(Error::Game7Error("Expected Game7 data for player".into()));
        };
        if card_playable(&(player7.hand[card as usize] as i32), board) {
            Ok(())
            
        } else {
            Err(Error::Game7Error("You can not play that card".into()))
        }
    }

    fn skip_turn(&mut self, sid: &str) -> Result<(), Error> {
        let board = &self.board;

        let Some(player_ref) = self.game_data.players.get(sid) else {
            return Err(Error::Game7Error("Player not found".into()));
        };

        let PlayerGameData::Player7(player7) = &player_ref.game else {
            return Err(Error::Game7Error("Expected Game7 data for player".into()));
        };

        if possible_skip(&player7.hand, &board) {
            self.turn_manager.advance_turn(&self.game_data.players);
            Ok(())
        } else {
            Err(Error::Game7Error("You can not skip right now".into()))
        }
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
    hand.iter()
        .any(|card| card_playable(&(*card as i32), board))
}
