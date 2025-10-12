use std::str::FromStr;

use rand::{Rng, rng};
use serde::Serialize;
use serde_json::json;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef},
    socket::Sid,
};

use crate::{
    helpers::count_point_score,
    models::{Lobby, Player, Player7Data, PlayerGameData, Players, TurnManager, TurnResponse},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Start7GameResponse<'a> {
    hand_info: &'a Vec<u32>,
    board: &'a Vec<Vec<i32>>,
    players_info: &'a Vec<SevenPlayerResponse<'a>>,
    turn: &'a TurnResponse,
    game_mode: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameOver7GameResponse {
    win_data: Vec<GameOverPlayer7GameResponse>,
}
impl GameOver7GameResponse {
    fn new(win_data: Vec<GameOverPlayer7GameResponse>) -> Self {
        Self { win_data }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameOverPlayer7GameResponse {
    name: String,
    round_score: u32,
    total_score: u32,
}
impl GameOverPlayer7GameResponse {
    fn new(name: String, round_score: u32, total_score: u32) -> Self {
        Self {
            name,
            round_score,
            total_score,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Hand7GameResponse<'a> {
    hand_info: &'a Vec<u32>,
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
    pub board: Vec<Vec<i32>>,
    pub r#box: Option<String>,
    pub turn_manager: TurnManager,
    pub starting_player_id: String,
}

impl<'a> Start7GameResponse<'a> {
    pub fn new(
        hand_info: &'a Vec<u32>,
        board: &'a Vec<Vec<i32>>,
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

// Responses
impl Game7Logic {
    pub fn response_move(&self, lobby: &Lobby, io: &SocketIo) {
        let turn_res = self.turn_manager.make_respone();

        let players_res: Vec<_> = lobby
            .players
            .get_all()
            .iter()
            .map(|player| SevenPlayerResponse::new(player))
            .collect();

        let _ = io.within(lobby.id.to_string()).broadcast().emit(
            "gameInfo",
            Start7GameResponse::new(&vec![], self.get_board(), &players_res, &turn_res),
        );
    }
    pub fn response_start(&self, lobby: &Lobby, io: &SocketIo) {
        let turn_res = self.turn_manager.make_respone();

        let players_res: Vec<_> = lobby
            .players
            .get_all()
            .iter()
            .map(|player| SevenPlayerResponse::new(player))
            .collect();

        lobby.players.get_all().iter().for_each(|player| {
            if let Some(s_cur) = io.get_socket(Sid::from_str(player.id.as_str()).unwrap()) {
                let PlayerGameData::Player7(player_7_data) = player.game.as_ref().unwrap() else {
                    unreachable!()
                };

                let _ = s_cur.emit(
                    "startedGame",
                    Start7GameResponse::new(
                        player_7_data.hand.as_ref(),
                        self.get_board(),
                        &players_res,
                        &turn_res,
                    ),
                );
            }
        });
    }
    pub fn response_hand(&self, lobby: &Lobby, s: &SocketRef) {
        let player = lobby.players.get(&s.id.to_string()).unwrap();

        let PlayerGameData::Player7(player_7_data) = player.game.as_ref().unwrap() else {
            unreachable!()
        };

        let payload = json!({
            "hand": player_7_data.hand,
        });

        let _ = s.emit("handInfo", payload);
    }

    pub fn response_game_over(&self, lobby: &Lobby, io: &SocketIo) {
        let response = GameOver7GameResponse::new(
            lobby
                .players
                .get_all()
                .iter()
                .map(|player| {
                    let PlayerGameData::Player7(player_7_data) = player.game.as_ref().unwrap()
                    else {
                        unreachable!()
                    };
                    GameOverPlayer7GameResponse::new(
                        player.name.to_string(),
                        count_point_score(&player_7_data.hand, 10, 15, 5),
                        player_7_data.total_score,
                    )
                })
                .collect(),
        );

        let _ = io
            .within(lobby.id.to_string())
            .broadcast()
            .emit("gameEnded", &response);
    }
}

impl Game7Logic {
    pub fn new(lobby: &mut Lobby, s: SocketRef, io: SocketIo) -> Self {
        let mut game = Self {
            board: vec![],
            turn_manager: TurnManager::new(),
            r#box: None,
            starting_player_id: String::new(),
        };

        lobby
            .players
            .get_mut_all()
            .iter_mut()
            .for_each(|player| player.game = Some(PlayerGameData::Player7(Player7Data::new())));

        game.start_game(lobby, s.id.to_string());
        game.response_start(lobby, &io);
        game
    }

    pub fn play_again(&mut self, lobby: &mut Lobby, s_id: String, io: SocketIo) {
        lobby.players.get_mut_all().iter_mut().for_each(|player| {
            if let Some(PlayerGameData::Player7(player_7_data)) = &mut player.game {
                player_7_data.play_again();
            }
        });
        self.start_game(lobby, s_id);
        self.response_move(lobby, &io);
        lobby.players.get_all().iter().for_each(|player| self.response_hand(lobby, &io.get_socket(Sid::from_str(player.id.as_str()).unwrap()).unwrap()));
    }

    pub fn play_card(&mut self, card: &i32) -> Result<(), String> {
        let suit = ((card - (card % 13)) / 13) as usize;
        let rank = card % 13 + 1;

        let row = if rank == 7 {
            1
        } else if rank < 7 {
            2
        } else {
            0
        };
        self.board[row][suit] = rank;

        Ok(())
    }


    fn start_game(&mut self, lobby: &mut Lobby, s_id: String) {
        println!(
            "Starting Game 7 with {} players",
            lobby.players.get_all().len()
        );

        self.turn_manager.update(s_id, &lobby.players);
        start_game_helper(self, lobby);

        println!("String player: {:?}", self.starting_player_id);

        self.turn_manager
            .update(self.starting_player_id.to_string(), &lobby.players);
    }

    fn get_board(&self) -> &Vec<Vec<i32>> {
        self.board.as_ref()
    }

    pub fn set_box(&mut self, id: String) {
        self.r#box = Some(id)
    }

    pub fn update_score(&self, players: &mut Players) {
        players.get_mut_all().iter_mut().for_each(|player| {
            if let Some(PlayerGameData::Player7(player_7_data)) = player.game.as_mut() {
                player_7_data.total_score += count_point_score(&player_7_data.hand, 10, 15, 5);
            } else {
                unreachable!()
            }
        })
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
        .update(data.starting_player_id.to_string(), &lobby.players);

    lobby
        .players
        .get_mut_all()
        .iter_mut()
        .for_each(|player| match player.game.as_mut().unwrap() {
            PlayerGameData::Player7(player) => player.set_cards_left(),
            _ => unreachable!(),
        });
}

fn deal_cards(data: &mut Game7Logic, lobby: &mut Lobby) {
    let mut card_deck: Vec<u32> = (14..=20).collect();
    let mut rng = rng();

    while !card_deck.is_empty() {
        let random_num = rng.random_range(0..card_deck.len() as u32) as usize;

        if card_deck[random_num] == 19 {
            data.starting_player_id = data.turn_manager.get_current().to_string();
        }

        let cur_id = data.turn_manager.get_current().to_string();
        if let Some(game) = lobby
            .players
            .get_mut(&cur_id)
            .and_then(|player| player.game.as_mut())
        {
            match game {
                PlayerGameData::Player7(player_data) => {
                    player_data.hand.push(card_deck[random_num]);
                }
                _ => unreachable!(),
            }
        }
        card_deck.remove(random_num);
        let _ = data.turn_manager.advance_turn(&lobby.players);
    }
}
