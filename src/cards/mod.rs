use self::card::Card;

pub mod card;
pub mod deck;
pub mod hand;
pub mod rank;
pub mod suit;

trait Match {
    // https://mastodon.technology/@bugaevc/102226891784062955
    fn is_all_same<T: PartialEq>(arr: &[T]) -> bool;
}

impl Match for Vec<Card> {
    // https://mastodon.technology/@bugaevc/102226891784062955
    fn is_all_same<T: PartialEq>(arr: &[T]) -> bool {
        arr.windows(2).all(|w| w[0] == w[1])
    }
}
