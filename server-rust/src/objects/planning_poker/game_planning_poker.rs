use std::collections::HashMap;

use crate::{
    objects::{lobby::lobby::Lobby, planning_poker::player_planning_poker::PlayerPlanningPoker},
    socket::{
        PlanningPokerPayload, planning_poker_socker::PlanningPokerEvents, send_error_socket::Error,
    },
};

#[derive(Debug, Clone)]
pub struct PlanningPoker {
    id: u32,
    host: String,
    players: Vec<PlayerPlanningPoker>,
    game_over: bool,
    card_set: HashMap<u32, String>,
}

impl From<Lobby> for PlanningPoker {
    fn from(value: Lobby) -> PlanningPoker {
        let new_players: Vec<PlayerPlanningPoker> = value
            .players
            .get_all()
            .iter()
            .map(PlayerPlanningPoker::from)
            .collect();
        PlanningPoker {
            id: value.id,
            host: value.host,
            players: new_players,
            game_over: false,
            card_set: HashMap::from([
                (0, "0".to_string()),
                (1, "1/4".to_string()),
                (2, "1/2".to_string()),
                (3, "1".to_string()),
                (4, "1.5".to_string()),
                (5, "2".to_string()),
                (6, "3".to_string()),
                (7, "4".to_string()),
                (8, "5".to_string()),
                (9, "7".to_string()),
                (10, "10".to_string()),
                (11, "14".to_string()),
                (12, "20".to_string()),
                (13, "∞".to_string()),
                (14, "?".to_string()),
                (15, "☕".to_string()),
            ]),
        }
    }
}
// Impls used on players
impl PlanningPoker {
    pub fn get_player(&self, key: &str) -> Option<&PlayerPlanningPoker> {
        self.players.iter().find(|p| p.id == key)
    }

    pub fn get_mut_player(&mut self, key: &str) -> Option<&mut PlayerPlanningPoker> {
        self.players.iter_mut().find(|player| player.id == key)
    }

    pub fn get_mut_player_result(&mut self, key: &str) -> Result<&mut PlayerPlanningPoker, Error> {
        match self.players.iter_mut().find(|player| player.id == key) {
            Some(p) => Ok(p),
            None => Err(Error::PlanningPokerError(format!(
                "Could not find player ID {}",
                key
            ))),
        }
    }
}

// Getters and Setters
impl PlanningPoker {
    pub fn get_game_over(&self) -> bool {
        self.game_over
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_players(&self) -> &Vec<PlayerPlanningPoker> {
        &self.players
    }

    pub fn get_card_set(&self) -> &HashMap<u32, String> {
        &self.card_set
    }
}

impl PlanningPoker {
    // pub fn set_up_game(&mut self) {
    //     self.players.iter_mut().for_each(|p| p.reset());
    //     self.game_over = false;
    // }

    pub fn handle_move(&mut self, data: PlanningPokerPayload, sid: &String) -> Result<(), Error> {
        match data.move_type {
            PlanningPokerEvents::PlayCard => self.play_card(data.card, sid),
            PlanningPokerEvents::PlayAnotherRound => self.play_another_round(sid),
        }
    }

    pub fn play_card(&mut self, played_card: u32, sid: &String) -> Result<(), Error> {
        if !self.card_set.contains_key(&played_card) {
            return Err(Error::PlanningPokerError(
                "You can not play that card, not in the deck".into(),
            ));
        }

        self.get_mut_player(sid)
            .ok_or_else(|| {
                Error::PlanningPokerError("Could not find your player ID in the game data".into())
            })?
            .card_played = Some(played_card);

        self.game_over = self.get_players().iter().all(|p| p.card_played.is_some()); // Set game over if all have played
        Ok(())
    }

    pub fn play_another_round(&mut self, _sid: &String) -> Result<(), Error> {
        self.players.iter_mut().for_each(|p| p.reset());
        self.game_over = false;
        Ok(())
    }
}
