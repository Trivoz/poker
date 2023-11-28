//! A card in the game of poker. This is of course excluding the jokers and the wild cards.
//!
//! Cards have a suit and a rank. The suit is one of the four suits in a deck of cards. The rank
//! is one of the 13 ranks in a deck of cards. The rank is also used to determine the value of the
//! card.

use super::deck::Deck;
use super::rank::Rank;
use super::suit::Suit;

/// A card in the game of poker.
#[derive(PartialEq, Debug)]
pub struct Card {
    /// The suit of the card.
    pub suit: Suit,
    /// The rank of the card.
    pub rank: Rank,
}

impl Card {
    /// Creates a new card.
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    /// Return a vector of all the cards in a deck of cards.
    pub fn all() -> Vec<Card> {
        // create a new deck of cards
        let mut cards = Deck::new();

        // return the cards
        cards.cards
    }

    fn get_suit(&self) -> Suit {
        self.suit
    }

    fn get_rank(&self) -> Rank {
        self.rank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let card = Card::new(Suit::Spades, Rank::Ace);
        assert_eq!(card.get_suit(), Suit::Spades);
        assert_eq!(card.get_rank(), Rank::Ace);
    }

    #[test]
    fn test_get_suit() {
        let card = Card::new(Suit::Spades, Rank::Ace);
        assert_eq!(card.get_suit(), Suit::Spades);
    }

    #[test]
    fn test_get_rank() {
        let card = Card::new(Suit::Spades, Rank::Ace);
        assert_eq!(card.get_rank(), Rank::Ace);
    }

    #[test]
    fn test_all() {
        let cards = Card::all();
        for (i, card) in Card::all().iter().enumerate() {
            assert_eq!(*card, cards[i]);
        }
    }
}
