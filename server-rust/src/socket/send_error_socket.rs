use colored::Colorize;
use serde::{Deserialize, Serialize};
use socketioxide::extract::SocketRef;

#[derive(Debug)]
pub enum Error {
    NotYourTurn(String),
    LobbyError(String),
    Game7Error(String),
    Game31Error(String),
    PlanningPokerError(String),
}

impl Error {
    pub fn emit_error_response(self, s: &SocketRef) {
        use Error::*;
        match self {
            NotYourTurn(err) => ErrorResponse::new(err, "Not your turn").emit(s),
            LobbyError(err) => ErrorResponse::new(err, "Lobby Error").emit(s),
            Game7Error(err) => ErrorResponse::new(err, "7 Game Error").emit(s),
            Game31Error(err) => ErrorResponse::new(err, "31 Game Error").emit(s),
            PlanningPokerError(err) => ErrorResponse::new(err, "Planning Poker Game Error").emit(s),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub r#type: String,
}

impl ErrorResponse {
    fn new(message: String, r#type: &str) -> Self {
        Self {
            message,
            r#type: r#type.to_string(),
        }
    }

    pub fn emit(self, s: &SocketRef) {
        // Use JSON formatting with serde_json for safety
        let json = serde_json::json!({
            "message": self.message,
            "type": self.r#type,
        });

        println!(
            "{}",
            format!(
                "[error emit] to: {} | event: errorMessage\n{}",
                s.id,
                serde_json::to_string_pretty(&json).unwrap_or_default()
            )
            .bright_black()
        );

        let _ = s.emit("errorMessage", &json);
    }
}
