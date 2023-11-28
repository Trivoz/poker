use super::card::Card;
use super::rank::Rank;
use super::suit::Suit;

/// A deck of cards.
pub struct Deck {
    /// The cards in the deck.
    pub cards: Vec<Card>,
}

impl Deck {
    /// Creates a new deck of cards.
    pub fn new() -> Self {
        let mut cards = Vec::new();
        let ranks = Rank::all();
        let suits = Suit::all();

        // Add the cards to the deck.
        for suit in suits.iter() {
            for rank in ranks.iter() {
                cards.push(Card::new(*suit, *rank));
            }
        }

        Self { cards }
    }
}
