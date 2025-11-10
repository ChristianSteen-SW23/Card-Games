use crate::{objects::Players, responses::TurnResponse, socket::send_error_socket::Error};

#[derive(Debug, Clone)]
pub struct TurnManager {
    current: String,
    next: String,
}

impl TurnManager {
    /// Builds a [`TurnResponse`] representing the current and next player.
    ///
    /// This is typically used when sending turn information to connected clients.
    ///
    /// # Returns
    /// A [`TurnResponse`] with the `current` and `next` player IDs.
    pub fn make_respone(&self) -> TurnResponse {
        TurnResponse {
            current: self.current.clone(),
            next: self.next.clone(),
        }
    }

    /// Creates a new, empty [`TurnManager`] with no current or next player set.
    ///
    /// # Example
    /// ```
    /// let manager = TurnManager::new();
    /// assert_eq!(manager.get_current(), "");
    /// ```
    pub fn new() -> Self {
        Self { current: String::new(), next: String::new() }
    }

    /// Updates the current player and determines who plays next.
    ///
    /// # Arguments
    /// * `current` - The ID of the player whose turn it currently is.
    /// * `players` - The collection of all players, used to find the next player in order.
    ///
    /// # Returns
    /// * `Ok(())` if the turn was successfully updated.
    /// * `Err(())` if the next player could not be determined.
    pub fn update(&mut self, current: String, players: &Players) -> Result<(), ()> {
        if let Some(next) = players.get_next(&current).and_then(|player| Some(player.id.to_string())) {
            self.current = current; 
            self.next = next; 
            Ok(())
        } else { Err(()) }
    }

    /// Returns the ID of the current player.
    ///
    /// # Returns
    /// A string slice containing the ID of the current player.
    pub fn get_current(&self) -> &str {
        &self.current
    }

    /// Returns the ID of the next player.
    ///
    /// # Returns
    /// A string slice containing the ID of the next player.
    pub fn get_next(&self) -> &str {
        &self.next
    }
    
    /// Advances the turn to the next player and recalculates the following player.
    ///
    /// This moves the turn forward in the player list by one position.
    ///
    /// # Arguments
    /// * `players` - The collection of all players, used to determine the new next player.
    ///
    /// # Returns
    /// * `Ok(())` if the turn advanced successfully.
    /// * `Err(())` if the next player could not be determined.
    pub fn advance_turn(&mut self, players: &Players) -> Result<(), ()> {
        self.update(self.next.to_string(), players)
    }

    /// Checks whether the given player is the one whose turn it currently is.
    ///
    /// If it is the player's turn, the function returns `Ok(())`.  
    /// Otherwise, it returns an [`Error::NotYourTurn`].
    ///
    /// # Arguments
    /// * `to_check` - The player ID to verify against the current turn.
    ///
    /// # Returns
    /// * `Ok(())` if it *is* the player's turn.
    /// * `Err(Error::NotYourTurn)` if it is *not* the player's turn.
    ///
    /// # Example
    /// ```
    /// let mut tm = TurnManager::new();
    /// tm.update("Alice".to_string(), &players).unwrap();
    /// assert!(tm.check_turn_with_error("Alice").is_ok());
    /// assert!(tm.check_turn_with_error("Bob").is_err());
    /// ```
    pub fn check_turn_with_error(&mut self, to_check: &str) -> Result<(), Error> {
        if self.get_current().eq(to_check) {
            Ok(())
        } else {
            Err(Error::NotYourTurn(String::from("It is not your turn")))
        }
    }
}

