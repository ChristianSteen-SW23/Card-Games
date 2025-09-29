use socketioxide::extract::SocketRef;

#[derive(Debug)]
pub struct ErrorResponse {
    pub message: String,
    pub type_message: String,
}

pub fn send_error_message(s: &SocketRef, err: ErrorResponse) {
    // Use JSON formatting with serde_json for safety
    let json = serde_json::json!({
        "message": err.message,
        "type": err.type_message,
    });

    let _ = s.emit("errorMessage", json);
}
