use rand::{rng, seq::SliceRandom};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter, Clone, Copy)]
enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

#[derive(Debug)]
struct Card {
    rank: char,
    suit: Suit,
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn shuffle(mut self) {
        self.cards.shuffle(&mut rng());
    }
    fn new() -> Self {
        Self::default()
    }
    fn with_decks(num_decks: u8) {
        let mut total_deck: Vec<Card> = vec![];
        for _ in 0..num_decks {
            total_deck.append(&mut Self::default().cards);
        }
    }
}

impl Default for Deck {
    fn default() -> Self {
        let mut deck: Vec<Card> = vec![];
        for suit in Suit::iter() {
            for i in 1u8..=13 {
                let rank = match i {
                    11 => 'J',
                    12 => 'Q',
                    13 => 'K',
                    _ => i as char,
                };
                deck.push(Card { rank, suit });
            }
        }
        Deck { cards: deck }
    }
}
