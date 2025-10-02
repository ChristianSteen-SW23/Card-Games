use serde::{Deserialize, Serialize};
use socketioxide::extract::SocketRef;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub r#type: String,
}

pub fn send_error_message(s: &SocketRef, err: ErrorResponse) {
    // Use JSON formatting with serde_json for safety
    let json = serde_json::json!({
        "message": err.message,
        "type": err.r#type,
    });

    let _ = s.emit("errorMessage", json);
}
