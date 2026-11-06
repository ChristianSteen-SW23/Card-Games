use crate::objects::{game7::Game7, game31::Game31, lobby::lobby::Lobby};


#[derive(Debug, Clone)]
pub enum GameLogic {
    Game7(Game7),
    Lobby(Lobby),
    Game31(Game31),
}

