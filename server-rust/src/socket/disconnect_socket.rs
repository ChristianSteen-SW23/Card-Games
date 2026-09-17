use colored::Colorize;
use socketioxide::{SocketIo, extract::SocketRef, socket::DisconnectReason};
use crate::state::SharedState;

pub fn disconnect_controller(
    s: SocketRef,
    reason: DisconnectReason,
    state: SharedState,
    io: SocketIo,
) {
    println!(
        "Socket {} on ns {} disconnected, reason: {:?}",
        s.id.to_string().green(),
        s.ns().to_string().blue(),
        reason.to_string().blue()
    );
    
    let Some(&room_id) = state.lock().unwrap().player_lobby.get(&s.id.to_string()) else {
        return;
    };

    if !state
        .lock()
        .unwrap()
        .lobbies
        .get(&room_id)
        .unwrap()
        .lock()
        .unwrap()
        .game_started
    {
        let _ = s.emit("leaveLobby", "data");
        state
            .lock()
            .unwrap()
            .delete_player(&room_id, s.id.to_string())
    } else {

        s.within(room_id.to_string())
            .broadcast()
            .emit("leaveLobby", "")
            .ok();
        let _ = io.leave(room_id.to_string());
        state.lock().unwrap().delete_room(&room_id)
    }
}
