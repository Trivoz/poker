use super::rank::Rank;

/// The suit of a card.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Suit {
    /// The suit of clubs.
    Clubs,
    /// The suit of diamonds.
    Diamonds,
    /// The suit of hearts.
    Hearts,
    /// The suit of spades.
    Spades,
}
impl Suit {
    /// Returns all the suits in a deck of cards.
    pub fn all() -> Vec<Suit> {
        vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
    }
}
