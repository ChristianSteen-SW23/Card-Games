use colored::Colorize;
use serde::{Deserialize, Serialize};
use socketioxide::{SocketIo, extract::SocketRef};

use crate::{
    models::{lobby, turn_manager, GameLogic, Lobby},
    state::SharedState,
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Game7Payload {
    pub card: Option<u32>,
    pub move_type: Game7Events,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Game7Events {
    PlayAgain,
    SkipTurn,
    PlayCard,
}

pub fn game_7_controller(s: SocketRef, data: Game7Payload, state: SharedState, io: SocketIo) {
    println!(
        "Game 7 Event:{} {}",
        format!("{:?}", data).blue(),
        s.id.to_string().green()
    );

    let state = state.lock().unwrap();

    let Some(lobby_arc) = state
        .lobbies
        .get(&state.player_lobby.get(&s.id.to_string()).unwrap())
    else {
        return;
    };
    let mut lobby = lobby_arc.lock().unwrap();

    if let Some(GameLogic::Game7Logic(game)) = &lobby.game {
        match Some(s.id.to_string()) != game.turn_manager.current && data.move_type != Game7Events::PlayAgain {
            true => {
                println!("It's not your turn!");
            }
            false => {
                println!("It's your turn!");
            }
        }
    }

    match data.move_type {
        Game7Events::PlayAgain => todo!(),
        Game7Events::SkipTurn => todo!(),
        Game7Events::PlayCard => todo!(),
    }

    // match data.game_mode {
    //     StartGameEvents::Seven => {
    //         lobby.game = Some(GameLogic::Game7Logic(Game7Logic::new(&mut lobby, s, io)));
    //     }
    //     StartGameEvents::ThirtyOne => todo!(),
    //     StartGameEvents::FiveHundred => todo!(),
    //     StartGameEvents::PlanningPoker => todo!(),
    // }
}

fn play_card(card: &u32, lobby: &Lobby, s: SocketRef) -> Result<(),String> {
    if card_playable(&card, &lobby) {
        todo!()
    }
    lobby.players.get(&s.id.to_string());

    Ok(())
}

fn card_playable(card: &u32, lobby: &Lobby) -> bool {

    true
}