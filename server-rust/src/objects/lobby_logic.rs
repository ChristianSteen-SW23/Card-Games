use crate::{objects::{GameData, Player, Players}, socket::{send_error_socket::Error, LobbyPayload}};


#[derive(Debug, Clone)]
pub struct LobbyLogic {
    game_data: GameData,
}


impl LobbyLogic {
    pub fn get_game_id(&self) -> u32 {
        self.game_data.id
    }

    pub fn get_host(&self) -> &str {
        &self.game_data.host
    }

    pub fn get_players(&self) -> &Players {
        &self.game_data.players
    }

    pub fn add_player(&mut self, sid: &String, username: Option<String>) -> Result<(),Error>{
        let player = Player::try_from((sid.to_string(), username.ok_or_else(|| Error::LobbyError("You need to send a username".to_string()))?))?;
        self.game_data.players.add(player);
        Ok(())
    }
}

impl TryFrom<(LobbyPayload, u32, String)> for LobbyLogic {
    type Error = Error;
    
    fn try_from(value: (LobbyPayload, u32, String)) -> Result<Self, Self::Error> {
        let (lobby_payload, new_lobby_id, sid) = value; 
        let username = lobby_payload.username.ok_or_else(|| Error::LobbyError("You need to send a username".to_string()))?;

        let player = Player::try_from((sid, username))?;
        
        Ok(Self {
            game_data: GameData::from((new_lobby_id, player))
        })
    }
}


// fn create_lobby(s: SocketRef, data: LobbyPayload, state: SharedState) {
//     let mut lobby = GameData::new(new_id, s.id.to_string());

//     match data.username {
//         Some(username) => {
//             if !check_username(&s, &username.to_string()) {
//                 return;
//             }

//             let player = Player::new(s.id.to_string(), username.to_string());
//             lobby.add_player(player);

//             state.add_lobby(lobby);
//             state.add_player_lobby(new_id, s.id.to_string());

//             let response = LobbyResponse {
//                 id: new_id,
//                 players: vec![PlayerResponse {
//                     playerid: s.id.to_string(),
//                     name: username,
//                     host: true,
//                 }],
//             };
//             s.join(new_id.to_string()).ok();
//             let _ = s.emit("conToLobby", response);
//         }
//         None => send_error_message(
//             &s,
//             ErrorResponse {
//                 message: "No username given".to_string(),
//                 r#type: "???".to_string(),
//             },
//         ),
//     }
// }