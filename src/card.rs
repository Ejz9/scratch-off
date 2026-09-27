#[derive(Debug)]
enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

#[derive(Debug)]
struct Card {
    rank: u8,
    suit: Suit,
}
