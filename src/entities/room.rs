use color_eyre::eyre;

use crate::entities::{dealer::Dealer, player::Player};

pub struct Room {
    //id
    players: Vec<Player>,
    dealer: Dealer,
}

pub fn create_room() -> eyre::Result<Room> {
    //room code?
    //create a "sever" local for peers to connect to
    //await player joins and return room on start (room accepts new players after this?)
    //
    todo!()
}

pub fn join_room(connection_string: &str) -> eyre::Result<Room> {
    //where is the room
    //connect player to the room
    //return room
    todo!()
}