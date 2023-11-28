//! The rank of a card, which is used to determine the value of the card.

use std::ops::Add;

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

impl Add for Rank {
    type Output = Option<Rank>;

    fn add(self, other: Rank) -> Option<Rank> {
        let mut rank = self as u8 + other as u8;
        if rank > 12 {
            rank -= 13;
        }
        match rank {
            0 => Some(Rank::King),
            1 => Some(Rank::Queen),
            2 => Some(Rank::Jack),
            3 => Some(Rank::Ten),
            4 => Some(Rank::Nine),
            5 => Some(Rank::Eight),
            6 => Some(Rank::Seven),
            7 => Some(Rank::Six),
            8 => Some(Rank::Five),
            9 => Some(Rank::Four),
            10 => Some(Rank::Three),
            11 => Some(Rank::Two),
            12 => Some(Rank::Ace),
            _ => None,
        }
    }
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
