use colored::Colorize;
use socketioxide::{SocketIo, extract::SocketRef, socket::DisconnectReason};

use crate::objects::states::SharedState;

pub fn disconnect_controller(
    s: SocketRef,
    reason: DisconnectReason,
    state: SharedState,
    io: SocketIo,
) {
    println!(
        "Socket {} on ns {} disconnected, reason: {}",
        s.id.to_string().green(),
        s.ns().to_string().blue(),
        reason.to_string().blue()
    );
    
    let Some(&room_id) = state.lock().unwrap().player_lobby_map.get(&s.id.to_string()) else {
        return;
    };

    let mut state = state.lock().unwrap();

    // state.game_map.get(&room_id);

    let _ = io.within(room_id.to_string()).emit("leaveLobby", "");
    let _ = io.leave(room_id.to_string());
    state.delete_room(&room_id);
    /* 
    if !state
        .lock()
        .unwrap()
        .game_map
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
    }*/
}
