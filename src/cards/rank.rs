//! The rank of a card, which is used to determine the value of the card.

/// The rank of a card, which is used to determine the value of the card.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Rank {
    /// The rank of king, which is the highest rank in a deck of cards.
    King,
    /// The rank of queen, which is the second highest rank in a deck of cards.
    Queen,
    /// The rank of jack, which is the third highest rank in a deck of cards.
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    /// The rank of ace, which is the lowest rank in a deck of cards.
    Ace,
}

impl Rank {
    /// Returns a vector of all the ranks in a deck of cards.
    pub fn all() -> Vec<Rank> {
        vec![
            Rank::King,
            Rank::Queen,
            Rank::Jack,
            Rank::Ten,
            Rank::Nine,
            Rank::Eight,
            Rank::Seven,
            Rank::Six,
            Rank::Five,
            Rank::Four,
            Rank::Three,
            Rank::Two,
            Rank::Ace,
        ]
    }
}
