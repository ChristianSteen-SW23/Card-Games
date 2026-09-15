use colored::Colorize;
use socketioxide::{SocketIo, extract::SocketRef, socket::DisconnectReason};

use crate::{
    objects::{GameLogic, lobby::lobby::Lobby, states::SharedState, traits::has_players::HasPlayers}, responses::{EmitContext, Event, LobbyResponse, Planned, Responses},
};

pub fn disconnect_controller(
    s: SocketRef,
    reason: DisconnectReason,
    state: SharedState,
    io: SocketIo,
) {
    let sid = s.id.to_string();
    println!(
        "Socket {} on ns {} disconnected, reason: {}",
        sid.green(),
        s.ns().to_string().blue(),
        reason.to_string().blue()
    );

    let Some(&room_id) = state.lock().unwrap().player_lobby_map.get(&sid) else {
        return;
    };

    let mut state = state.lock().unwrap();

    let Some(game_arc) = state.game_map.get(&room_id) else {
        return;
    };

    let mut game_guard = game_arc.lock().unwrap();
    let player_count = (&*game_guard.player_ids()).len();
    match (&mut *game_guard, player_count) {
        (GameLogic::Lobby(_), 1) => {
            drop(game_guard);
            let _ = io.within(room_id.to_string()).emit("leaveLobby", "");
            let _ = io.leave(room_id.to_string());
            state.delete_room(&room_id);
        }
        (GameLogic::Lobby(lobby), _) => {
            lobby.remove_player(&sid);
            send_lobby_update(lobby,&io,room_id);
            drop(game_guard);
            state.delete_player_from_player_map(sid);
        }
        _ => {
            drop(game_guard);
            let _ = io.within(room_id.to_string()).emit("leaveLobby", "");
            let _ = io.leave(room_id.to_string());
            state.delete_room(&room_id);
        }
    }
}

fn send_lobby_update<'a>(lobby: &'a Lobby, io: &'a SocketIo, lobby_id: u32) {
    Responses::Single(Planned::new(
        Event::UpdateLobby,
        EmitContext::Room {
            io,
            room_id: lobby_id,
        },
        &LobbyResponse::from(&*lobby).to_owned(),
    )).emit_all();
}
