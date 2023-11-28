//! In poker, the core part, or at least one of the main parts,
//! is what 'hand'  you have. this is important becuase it can determine who wins and loses.
//!
//! this module contains things that pertain to the functionality of the aforementioned subject
//! matter.

use super::card::Card;
use super::rank::Rank;
use super::suit::Suit;

/// A struct representing a hand of cards in a game of poker.
///
/// # Example
/// ```rust,ignore,no_run
/// use poker::Hand;
///
/// let hand = Hand::new();
///
/// if hand.is_flush() {
///    println!("You have a flush!");
///    // Do something
/// }
/// // I'm sure you can think of something better to do with this.
/// ```
pub enum Hand {
    /// A Royal Flush is a straight flush with the highest cards.
    RoyalFlush,
    /// A Straight Flush is a flush with cards in a sequence.
    StraightFlush,
    /// A Four of a Kind is four cards of the same rank.
    FourOfAKind,
    /// A Full House is three cards of the same rank and two cards of the same rank.
    FullHouse,
    /// A Flush is five cards of the same suit.
    Flush,
    /// A Straight is five cards in a sequence.
    Straight,
    /// A Three of a Kind is three cards of the same rank.
    ThreeOfAKind,
    /// A Two Pair is two cards of the same rank and two cards of the same rank.
    TwoPair,
    /// A One Pair is two cards of the same rank.
    OnePair,
    /// A High Card is a hand that does not fit into any of the other categories.
    HighCard,
}

impl PartialEq for Hand {
    /// Checks if two hands are equal.
    ///
    /// # Example
    ///
    /// ```rust,ignore,no_run
    /// use poker::Hand;
    ///
    /// let hand = Hand::new();
    ///
    /// if hand == Hand::RoyalFlush {
    ///   println!("You have a royal flush!");
    ///   // Do something
    /// }
    fn eq(&self, other: &Hand) -> bool {
        match self {
            Hand::RoyalFlush => matches!(other, Hand::RoyalFlush),
            Hand::StraightFlush => matches!(other, Hand::StraightFlush),
            Hand::FourOfAKind => matches!(other, Hand::FourOfAKind),
            Hand::FullHouse => matches!(other, Hand::FullHouse),
            Hand::Flush => matches!(other, Hand::Flush),
            Hand::Straight => matches!(other, Hand::Straight),
            Hand::ThreeOfAKind => matches!(other, Hand::ThreeOfAKind),
            Hand::TwoPair => matches!(other, Hand::TwoPair),
            Hand::OnePair => matches!(other, Hand::OnePair),
            Hand::HighCard => matches!(other, Hand::HighCard),
        }
    }
}

impl Default for Hand {
    /// Creates a new hand.
    ///
    /// # Example
    /// ```rust,ignore,no_run
    /// use poker::Hand;
    ///
    /// let hand = Hand::new();
    /// ```
    fn default() -> Hand {
        Hand::new()
    }
}

impl Hand {
    /// Creates a new hand.
    ///
    /// # Example
    /// ```rust,ignore,no_run
    /// use poker::Hand;
    ///
    /// let hand = Hand::new();
    /// ```
    pub fn new() -> Hand {
        Hand::HighCard
    }

    /// Returns the name of the hand.
    ///
    /// # Example
    ///
    /// ```rust,ignore,no_run
    /// use poker::Hand;
    ///
    /// let hand = Hand::new();
    ///
    /// println!("You have a {}!", hand.name());
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Hand::RoyalFlush => "Royal Flush",
            Hand::StraightFlush => "Straight Flush",
            Hand::FourOfAKind => "Four of a Kind",
            Hand::FullHouse => "Full House",
            Hand::Flush => "Flush",
            Hand::Straight => "Straight",
            Hand::ThreeOfAKind => "Three of a Kind",
            Hand::TwoPair => "Two Pair",
            Hand::OnePair => "One Pair",
            Hand::HighCard => "High Card",
        }
    }

    /// Checks if the hand is a Royal Flush.
    ///
    /// Pattern:
    /// - All cards are of the same suit.
    /// - All cards are of the same rank.
    /// - The cards are an Ace, King, Queen, Jack, and Ten.
    /// - The cards are in a sequence.
    pub fn is_royal_flush(&self, cards: Vec<Card>) -> bool {
        let mut cards = cards;

        // By default, the hand is not a royal flush.
        let mut _is_flush = false;

        // Check if all cards are of the same suit.
        true
    }

    /// Checks if the hand is a Straight Flush.
    pub fn is_straight_flush(&self) -> bool {
        self == &Hand::StraightFlush
    }

    /// Checks if the hand is a Four of a Kind.
    pub fn is_four_of_a_kind(&self) -> bool {
        self == &Hand::FourOfAKind
    }

    /// Checks if the hand is a Full House.
    pub fn is_full_house(&self) -> bool {
        self == &Hand::FullHouse
    }

    /// Checks if the hand is a Flush.
    pub fn is_flush(&self) -> bool {
        self == &Hand::Flush
    }

    /// Checks if the hand is a Straight.
    pub fn is_straight(&self) -> bool {
        self == &Hand::Straight
    }

    /// Checks if the hand is a Three of a Kind.
    pub fn is_three_of_a_kind(&self) -> bool {
        self == &Hand::ThreeOfAKind
    }

    /// Checks if the hand is a Two Pair.
    pub fn is_two_pair(&self) -> bool {
        self == &Hand::TwoPair
    }

    /// Checks if the hand is a One Pair.
    pub fn is_one_pair(&self) -> bool {
        self == &Hand::OnePair
    }

    /// Checks if the hand is a High Card.
    pub fn is_high_card(&self) -> bool {
        self == &Hand::HighCard
    }
}
