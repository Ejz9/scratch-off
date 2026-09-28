use color_eyre::eyre;

use crate::entities::room::{create_room, join_room};

mod deck;
mod entities;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    //create room or join room
    let intention: bool = true;
    let room = match intention {
        true => create_room()?,
        false => {
            let connection_string = "peer_address";
            join_room(connection_string)?
        }
    };

    //lobby in room?
    todo!()
}
