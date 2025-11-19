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
        let Some(player_ref) = self.game_data.players.get_mut(sid) else {
            return Err(Error::Game7Error("Player not found".into()));
        };
        let PlayerGameData::Player7(player7) = &mut player_ref.game else {
            return Err(Error::Game7Error("Expected Game7 data for player".into()));
        };

        if !card_playable(&(card), board) {
            return Err(Error::Game7Error("You can not play that card".into()));
        }
        if !player7.hand.contains(&(card as u32)) {
            return Err(Error::Game7Error(
                "You do not have that card in your hand".into(),
            ));
        }
        let Some(pos) = player7.hand.iter().position(|c| *c == card as u32) else {
            return Err(Error::Game7Error("Failed to remove card".into()));
        };
        player7.hand.remove(pos);

        let suit = ((card - (card % 13)) / 13) as usize;
        let rank = card % 13 + 1;
        if rank == 7 {
            self.board[1][suit] = rank;
        } else if rank > 7 {
            self.board[0][suit] = rank;
        } else if rank < 7 {
            self.board[2][suit] = rank;
        }

        self.turn_manager
            .advance_turn(&self.game_data.players)
            .map_err(|_| Error::Game7Error("Could not advance turn".into()))?;

        Ok(())
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
    !hand.iter()
        .any(|card| card_playable(&(*card as i32), board))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_playable_empty_board() {
        let board: Vec<Vec<i32>> = vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]];
        
        // Only card 19 (7 of hearts) should be playable on empty board
        assert!(card_playable(&19, &board));
        assert!(!card_playable(&18, &board));
        assert!(!card_playable(&20, &board));
        assert!(!card_playable(&0, &board));
        assert!(!card_playable(&6, &board));
        assert!(!card_playable(&7, &board));
        assert!(!card_playable(&32, &board));
        assert!(!card_playable(&45, &board));
    }

    #[test]
    fn test_card_playable_19_played() {
        let board: Vec<Vec<i32>> = vec![vec![0, 0, 0, 0], vec![0, 7, 0, 0], vec![0, 0, 0, 0]];
        
        // After 7 of hearts is played, 6 and 8 of hearts should be playable
        assert!(card_playable(&18, &board)); // 6 of hearts
        assert!(card_playable(&20, &board)); // 8 of hearts
        
        // Other 7s should be playable
        assert!(card_playable(&6, &board));   // 7 of clubs
        assert!(card_playable(&32, &board));  // 7 of diamonds
        assert!(card_playable(&45, &board));  // 7 of spades
        
        // Cards not adjacent to any played cards should not be playable
        assert!(!card_playable(&0, &board));  // Ace of clubs
        assert!(!card_playable(&17, &board)); // 5 of hearts
        assert!(!card_playable(&21, &board)); // 9 of hearts
    }

    #[test]
    fn test_card_playable_all_7s_played() {
        let board: Vec<Vec<i32>> = vec![vec![0, 0, 0, 0], vec![7, 7, 7, 7], vec![0, 0, 0, 0]];
        
        // 6s and 8s of all suits should be playable
        assert!(card_playable(&5, &board));   // 6 of clubs
        assert!(card_playable(&7, &board));   // 8 of clubs
        assert!(card_playable(&18, &board));  // 6 of hearts
        assert!(card_playable(&20, &board));  // 8 of hearts
        assert!(card_playable(&31, &board));  // 6 of diamonds
        assert!(card_playable(&33, &board));  // 8 of diamonds
        assert!(card_playable(&44, &board));  // 6 of spades
        assert!(card_playable(&46, &board));  // 8 of spades
        
        // Other cards should not be playable yet
        assert!(!card_playable(&4, &board));  // 5 of clubs
        assert!(!card_playable(&8, &board));  // 9 of clubs
        assert!(!card_playable(&17, &board)); // 5 of hearts
        assert!(!card_playable(&21, &board)); // 9 of hearts
    }

    #[test]
    fn test_card_playable_sequence() {
        // Test building a sequence up from 7
        // Card 6 = 7 of clubs, card 7 = 8 of clubs, card 8 = 9 of clubs
        // After playing 7 of hearts, 7 of clubs, and 8 of clubs
        // board[0][0] = 8 (rank 8 is highest > 7 in clubs)
        // board[1][1] = 7 (7 of hearts must be played first)
        let board: Vec<Vec<i32>> = vec![vec![8, 0, 0, 0], vec![7, 7, 0, 0], vec![0, 0, 0, 0]];
        
        // Card 8 (9 of clubs, rank 9) should be playable (next in sequence after rank 8)
        assert!(card_playable(&8, &board));
        
        // Card 9 (10 of clubs, rank 10) should not be playable (not next in sequence)
        assert!(!card_playable(&9, &board));
    }

    #[test]
    fn test_card_playable_sequence_down() {
        // Test building a sequence down from 7
        // Card 5 = 6 of clubs, card 4 = 5 of clubs
        // After playing 7 of hearts, 7 of clubs, and 6 of clubs
        // board[2][0] = 6 (rank 6 is lowest < 7 in clubs)
        // board[1][1] = 7 (7 of hearts must be played first)
        let board: Vec<Vec<i32>> = vec![vec![0, 0, 0, 0], vec![7, 7, 0, 0], vec![6, 0, 0, 0]];
        
        // Card 4 (5 of clubs, rank 5) should be playable (next in sequence down from rank 6)
        assert!(card_playable(&4, &board));
        
        // Card 3 (4 of clubs, rank 4) should not be playable (not next in sequence)
        assert!(!card_playable(&3, &board));
    }

    #[test]
    fn test_possible_skip_no_playable_cards() {
        let board: Vec<Vec<i32>> = vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]];
        let hand: Vec<u32> = vec![0, 1, 2, 3]; // No 7 of hearts
        
        // Should be able to skip when no cards are playable
        assert!(possible_skip(&hand, &board));
    }

    #[test]
    fn test_possible_skip_with_playable_cards() {
        let board: Vec<Vec<i32>> = vec![vec![0, 0, 0, 0], vec![0, 7, 0, 0], vec![0, 0, 0, 0]];
        let hand: Vec<u32> = vec![18, 20]; // 6 and 8 of hearts - both playable
        
        // Should not be able to skip when holding playable cards
        assert!(!possible_skip(&hand, &board));
    }

    #[test]
    fn test_possible_skip_mixed_cards() {
        let board: Vec<Vec<i32>> = vec![vec![0, 0, 0, 0], vec![0, 7, 0, 0], vec![0, 0, 0, 0]];
        let hand: Vec<u32> = vec![0, 1, 2, 20]; // Only 8 of hearts is playable
        
        // Should not be able to skip even if only one card is playable
        assert!(!possible_skip(&hand, &board));
    }

    #[test]
    fn test_card_playable_edge_cases() {
        // Test king (highest card in sequence)
        // Card 11 = Queen of Clubs (rank 12), Card 12 = King of Clubs (rank 13)
        // After playing 7 of hearts, then 7, 8, 9, 10, J, Q of clubs
        // board[0][0] = 12 (Queen is highest in clubs)
        // board[1][1] = 7 (7 of hearts must be played first)
        let board: Vec<Vec<i32>> = vec![vec![12, 0, 0, 0], vec![7, 7, 0, 0], vec![0, 0, 0, 0]];
        
        // Card 12 (King, rank 13) should be playable when Queen (rank 12) is on board
        assert!(card_playable(&12, &board));
    }

    #[test]
    fn test_card_playable_ace() {
        // Test ace (lowest card in sequence)
        // Card 0 = Ace of Clubs (rank 1), Card 1 = 2 of Clubs (rank 2)
        // After playing 7 of hearts, then 7, 6, 5, 4, 3, 2 of clubs
        // board[2][0] = 2 (rank 2 is lowest in clubs)
        // board[1][1] = 7 (7 of hearts must be played first)
        let board: Vec<Vec<i32>> = vec![vec![0, 0, 0, 0], vec![7, 7, 0, 0], vec![2, 0, 0, 0]];
        
        // Card 0 (Ace, rank 1) should be playable when 2 (rank 2) is on board
        assert!(card_playable(&0, &board));
    }
}
