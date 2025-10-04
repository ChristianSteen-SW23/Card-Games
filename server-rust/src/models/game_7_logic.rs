use std::str::FromStr;

use rand::{Rng, rng};
use serde::{Serialize};
use socketioxide::{SocketIo, extract::SocketRef, socket::Sid};

use crate::models::{
    Lobby, Player, Player7Data, PlayerGameData, TurnManager, TurnResponse,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Start7GameResponse<'a> {
    hand_info: &'a Vec<u32>,
    board: &'a Vec<Vec<u32>>,
    players_info: &'a Vec<SevenPlayerResponse<'a>>,
    turn: &'a TurnResponse,
    game_mode: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SevenPlayerResponse<'a> {
    name: &'a String,
    id: &'a String,
    cards_left: usize,
}

#[derive(Debug, Clone)]
pub struct Game7Logic {
    pub board: Vec<Vec<u32>>,
    pub r#box: Option<String>,
    pub turn_manager: TurnManager,
    pub starting_player_id: String,
}

impl<'a> Start7GameResponse<'a> {
    pub fn new(
        hand_info: &'a Vec<u32>,
        board: &'a Vec<Vec<u32>>,
        players: &'a Vec<SevenPlayerResponse>,
        turn: &'a TurnResponse,
    ) -> Self {
        Self {
            hand_info,
            board,
            players_info: players,
            turn,
            game_mode: "7".to_string(),
        }
    }
}

impl<'a> SevenPlayerResponse<'a> {
    pub fn new(player: &'a Player) -> Self {
        let cards_left = {
            let PlayerGameData::Player7(player) = player.game.clone().unwrap() else {
                unreachable!()
            };
            player.cards_left
        };
        Self {
            name: &player.name,
            id: &player.id,
            cards_left,
        }
    }
}

impl Game7Logic {
    pub fn new(lobby: &mut Lobby, s: SocketRef, io: SocketIo) -> Self {
        let mut game = Self {
            board: vec![],
            turn_manager: TurnManager::new(),
            r#box: None,
            starting_player_id: "".to_string(),
        };

        game.start_game(lobby, s, io);

        game
    }

    fn start_game(&mut self, lobby: &mut Lobby, s: SocketRef, io: SocketIo) {
        println!("Starting Game 7 with {} players", lobby.players.len());

        lobby
            .players
            .iter_mut()
            .for_each(|player| player.game = Some(PlayerGameData::Player7(Player7Data::new())));

        self.turn_manager.set_current(s.id.to_string());
        start_game_helper(self, lobby);

        self.turn_manager
            .set_current(self.starting_player_id.clone());
        self.turn_manager.set_next(&lobby.players);

        // Return data to player
        let turn_res = self.turn_manager.make_respone();
        let players_res = lobby
            .players
            .iter()
            .map(SevenPlayerResponse::new)
            .collect::<Vec<_>>();

        lobby.players.iter().for_each(|player| {
            if let Some(s_cur) = io.get_socket(Sid::from_str(player.id.as_str()).unwrap()) {
                let PlayerGameData::Player7(player_7_data) = player.game.as_ref().unwrap() else {
                    unreachable!()
                };

                let _ = s_cur.emit(
                    "startedGame",
                    Start7GameResponse::new(player_7_data.hand.as_ref(), self.get_board(), &players_res, &turn_res),
                );
            }
        });
    }

    fn get_board (&self) -> &Vec<Vec<u32>>{
        self.board.as_ref()
    }

    fn handle_move(&mut self, player_id: &str, data: serde_json::Value) {
        println!("Player {} made a move: {:?}", player_id, data);
    }
}

fn start_game_helper(data: &mut Game7Logic, lobby: &mut Lobby) {
    data.r#box = None;
    let rows = 3;
    let cols = 4;
    data.board = vec![vec![0; cols]; rows];

    data.turn_manager.advance_turn(&(lobby.players));
    deal_cards(data, lobby);
    data.turn_manager
        .set_current(data.starting_player_id.clone());
    data.turn_manager.set_next(&(lobby.players));

    lobby
        .players
        .iter_mut()
        .for_each(|player| match player.game.as_mut().unwrap() {
            PlayerGameData::Player7(player) => player.set_cards_left(),
            _ => unreachable!(),
        });
}

fn deal_cards(data: &mut Game7Logic, lobby: &mut Lobby) {
    let mut card_deck: Vec<u32> = (1..=52).collect();

    let mut rng = rng();
    let mut i = 0;
    while card_deck.len() != 0 {
        let random_num = rng.random_range(0..(card_deck.len() as u32));

        if card_deck[random_num as usize] == 19 {
            data.starting_player_id = data.turn_manager.get_current().unwrap().to_string();
        }
        if let Some(game) = &mut lobby.players[i].game {
            match game {
                PlayerGameData::Player7(player) => {
                    player.hand.push(card_deck[random_num as usize]);
                }
                _ => unreachable!(),
            }
        }
        card_deck.remove(random_num as usize);
        i = (i + 1) % lobby.players.len();
    }
}
