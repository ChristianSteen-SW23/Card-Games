use serde::{Deserialize, Serialize};
use socketioxide::extract::SocketRef;

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

        let _ = s.emit("errorMessage", json);
    }
}

pub enum Error {
    NotYourTurn(String),
    LobbyError(String),
}

impl Error {
    pub fn emit_error_response(self, s: &SocketRef) {
        use Error::*;
        match self {
            NotYourTurn(err) => ErrorResponse::new(err, "Not your turn").emit(s),
            LobbyError(err) => ErrorResponse::new(err, "Lobby Error").emit(s),
        }
    }
}
