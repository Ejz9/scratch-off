use crate::deck::Card;

pub struct Player {
    id: u32,
    name: String,
    chips: u64,
    cards: Option<PlayerHand>,
}

pub struct PlayerHand {
    primary: Card,
    secondary: Card,
}
