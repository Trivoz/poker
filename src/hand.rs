//! In poker, the core part, or at least one of the main parts,
//! is what 'hand'  you have. this is important becuase it can determine who wins and loses.
//!
//! this module contains things that pertain to the functionality of the aforementioned subject
//! matter.

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
