use std::sync::Arc;

use colored::Colorize;
use socketioxide::{SocketIo, extract::SocketRef, socket::DisconnectReason};

use crate::{models::Player, state::SharedState};

pub fn disconnect_controller(s: SocketRef, reason: DisconnectReason, state: SharedState) {
    println!(
        "Socket {} on ns {} disconnected, reason: {:?}",
        s.id.to_string().green(),
        s.ns().blue(),
        reason.to_string().blue()
    );

    let mut state = state.lock().unwrap();

    if let Some(&roomID) = state.player_lobby.get(&s.id.to_string()) {
        {
            let lobby_data = state.lobbies.get(&roomID).unwrap().lock().unwrap(); // Unwrap is safe to do so here
            if !lobby_data.game_started {
                drop(lobby_data);
                // TODO! Fix so the player is deleted
                s.emit("leaveLobby", "data");
                state.delete_player(&roomID, s.id.to_string());
                println!("{:?}",state);
                return;
            }

            s.within(roomID.to_string())
                .broadcast()
                .emit("leaveLobby", "")
                .ok();
        }
        // SHUTDOWN ROOM HERE
        // TODO! Få smit folk ud af deres socket IO room
        // Jeg skal have fundet ud af hvordan får IO her ind ellers kan jeg ikke smide dem ud
        state.delete_room(&roomID);
    }
}
