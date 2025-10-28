use crate::{objects::Players, responses::TurnResponse};

#[derive(Debug, Clone)]
pub struct TurnManager {
    current: String,
    next: String,
}

impl TurnManager {
    /// ✅ Builds a `TurnResponse` object representing the current and next player.
    /// Useful for sending turn info to clients.
    pub fn make_respone(&self) -> TurnResponse {
        TurnResponse {
            current: self.current.clone(),
            next: self.next.clone(),
        }
    }

    /// 🆕 Creates a new, empty `TurnManager` with no current or next player set.
    pub fn new() -> Self {
        Self { current: String::new(), next: String::new() }
    }

    /// 🔄 Updates the current player and calculates the next player based on the provided `players` list.
    /// 
    /// # Arguments
    /// * `current` - The ID of the player whose turn it currently is.
    /// * `players` - The collection of all players to determine the next player.
    /// 
    /// # Returns
    /// * `Ok(())` if a next player was found and turn was updated successfully.  
    /// * `Err(())` if no next player could be determined.
    pub fn update(&mut self, current: String, players: &Players) -> Result<(), ()> {
        if let Some(next) = players.get_next(&current).and_then(|player| Some(player.id.to_string())) {
            self.current = current; 
            self.next = next; 
            Ok(())
        } else { Err(()) }
    }

    /// 🎯 Returns the current player's ID as a string slice.
    pub fn get_current(&self) -> &str {
        &self.current
    }

    /// ⏭️ Returns the next player's ID as a string slice.
    pub fn get_next(&self) -> &str {
        &self.next
    }
    
    /// ⏩ Advances the turn to the next player and recalculates who’s next afterwards.
    /// 
    /// # Returns
    /// * `Ok(())` if turn advanced successfully.  
    /// * `Err(())` if the next player could not be determined.
    pub fn advance_turn(&mut self, players: &Players) -> Result<(), ()> {
        self.update(self.next.to_string(), players)
    }
}
