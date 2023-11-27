//!    Copyright 2023 Joshua Rose
//!
//!Licensed under the Apache License, Version 2.0 (the "License");
//!you may not use this file except in compliance with the License.
//!You may obtain a copy of the License at
//!
//!  http://www.apache.org/licenses/LICENSE-2.0
//!
//!Unless required by applicable law or agreed to in writing, software
//!distributed under the License is distributed on an "AS IS" BASIS,
//!WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//!See the License for the specific language governing permissions and
//!limitations under the License.
//! A library for playing poker. No implementation yet. (WIP)
//!
//! # Example
//!
//! ```rust,ignore,no_run
//! use poker::Hand;
//!
//! let hand = Hand::new();
//! let hand2 = Hand::new();
//!
//! let winner = hand.compare(hand2);
//! println!("The winner is: {}", winner);
//!
//! // Some other stuff idk
//! ```
#![allow(unused)]

use rand::Rng;
extern crate strum;

use std::{fmt, process::Command};

use rand::seq::SliceRandom;

use strum::{EnumIter, IntoEnumIterator};

/// The minimum amount of players in a game of poker.
const MIN_PLAYERS: u8 = 1;

/// The maximum amount of players in a game of poker.
const MAX_PLAYERS: u8 = 4;

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
enum Hand {
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

#[derive(Clone, Debug, EnumIter)]
/// An enum of names for the AI players.
///
/// Note: This is a [`strum`] enum, and derives the [`EnumIter`] trait.
///
/// [`strum`]: https://docs.rs/strum/0.19.3/strum/
/// [`EnumIter`]: https://docs.rs/strum/0.19.3/strum/trait.EnumIter.html
enum Name {
    /// Charles is a good player and will always win.
    Charles,

    /// John is a decent player and will win most of the time.
    ///
    /// He is the default player.
    John,

    /// Ted is a bad player and will lose most of the time.
    Ted,

    /// Jeffery is a terrible player and will always lose.
    Jeffery,
}

impl Default for Name {
    /// Get the default name of a list of AI players.
    ///
    /// # Returns
    ///
    /// * `Name` - The default name. (John in this case)
    fn default() -> Name {
        Name::John
    }
}

impl fmt::Display for Name {
    /// Output the name of an AI player.
    ///
    /// # Returns
    ///
    /// * `String` - The formatted name.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Name::Charles => write!(f, "Charles"),
            Name::John => write!(f, "John"),
            Name::Ted => write!(f, "Ted"),
            Name::Jeffery => write!(f, "Jeffery"),
        }
    }
}

impl PartialEq for Name {
    /// Compare two names.
    ///
    /// # Arguments
    ///
    /// * `other` - The other name to compare.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether or not the names are equal.
    fn eq(&self, other: &Self) -> bool {
        match self {
            Name::Charles => other == &Name::Charles,
            Name::John => other == &Name::John,
            Name::Ted => other == &Name::Ted,
            Name::Jeffery => other == &Name::Jeffery,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Name {
    /// Get a random name from the [`Name`] enum.
    ///
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount of names to get.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Name>, &'static str>` - A vector of [`Name`]s.
    ///
    /// # Example
    ///
    /// ```rust,ignore,no_run
    /// use poker::Name;
    ///
    /// let names: Vec<Name> = Name::get_random.unwrap();
    /// println!("{:?}", names);
    ///
    /// // [Charles, John, Ted] (randomly generated)
    /// ```
    ///
    /// # Panics
    ///
    /// * If Amount is exceeds [`Name::iter().count()`].
    ///
    /// [`Name`]: enum.Name.html
    /// [`Name::iter().count()`]: enum.Name.html#method.iter
    fn get_random(amount: u8) -> Result<Vec<Name>, &'static str> {
        let names: Vec<Name> = Name::iter().collect::<Vec<Name>>();
        let sample: Vec<Name> = names
            .choose_multiple(&mut rand::thread_rng(), amount as usize)
            .cloned()
            .collect();
        Ok(sample)
    }

    /// Get the skill of an AI player.
    ///
    /// # Returns
    ///
    /// * `u8` - The skill of the AI player.
    fn assign_skill(&self) -> u8 {
        match self {
            Name::Charles => 100, // 100% win chance
            Name::John => 75,     // 75%  win chance
            Name::Ted => 25,      // 25%  win chance
            Name::Jeffery => 0,   // 0%   win chance
        }
    }

    /// Announce a message from an AI player.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the AI player.
    /// * `message` - The message to announce.
    ///
    /// # Example
    ///
    /// ```rust,ignore,no_run
    /// use poker::Name;
    ///
    /// Name::speak(Name::John, "Hello, world!");
    ///
    /// // John: Hello, world!
    /// ```
    ///
    fn speak(name: Name, message: &str) {
        println!("{}: {}", name, message);
    }
}

/// Ask the user how many AI players they would like to play against.
///
/// # Returns
///
/// * `u8` - A selection of how many AI players to play against between 1 and 4.
///
/// # Panics
///
/// * If the user does not enter a number between 1 and 4.
fn ask_amount() -> u8 {
    println!("How many players would you like to compete against? (1-4)");
    let mut amount = String::new();
    std::io::stdin().read_line(&mut amount).unwrap();
    let amount: u8 = amount.trim().parse().unwrap_or_else(|_| {
        panic!(
            "Please enter a number between {} and {}",
            MIN_PLAYERS, MAX_PLAYERS
        )
    });

    if !(MIN_PLAYERS..=MAX_PLAYERS).contains(&amount) {
        panic!(
            "Please enter a number between {} and {}",
            MIN_PLAYERS, MAX_PLAYERS
        );
    }

    amount
}

/// Ask the user if they would like to see the rules of poker.
///
/// # Returns
///
/// * `bool` - Whether or not the user would like to see the rules of poker.
///
/// # Panics
///
/// * If the user does not enter y or n.
fn ask_show_rules() -> bool {
    let mut answer = String::new();

    println!("Would you like to see the rules of poker before you play? (y/N)");
    std::io::stdin().read_line(&mut answer).unwrap();
    let mut show_rules: String = answer
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("{}", String::from("Please enter y or n")));
    show_rules = show_rules.to_lowercase();

    if ["y", "n"].contains(&&show_rules[..]) {
        return show_rules == "y";
    }
    panic!("Please enter y or n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_skill() {
        let name = Name::John;
        assert_eq!(name.assign_skill(), 75);
    }

    #[test]
    fn test_speak() {
        let name = Name::John;
        Name::speak(name.clone(), "Hello, world!");
    }

    #[test]
    fn test_name_display() {
        let name = Name::John;
        assert_eq!(format!("{}", name), "John");
    }

    #[test]
    fn test_name_iter() {
        let mut names: Vec<Name> = Vec::new();
        for name in Name::iter() {
            names.push(name);
        }
        assert_eq!(names.len(), 4);
    }

    #[test]
    fn test_name_iter_count() {
        assert_eq!(Name::iter().count(), 4);
    }
}

/// Clear the terminal.
///
/// # Panics
///
/// * If the terminal cannot be cleared, in this case, if the `clear` command is not found.
fn clear_terminal() {
    Command::new("clear")
        .status()
        .expect("Unable to clear terminal, please clear manually");
}

/// Read the rules of poker from a file.
///
/// # Panics
///
/// * If the file cannot be found.
fn read_rules_file() {
    let rules = std::fs::read_to_string("rules.txt").expect("Unable to find rules.txt");
    println!("{}", rules);
}

/// The main function.
fn main() {
    // Initially, clear the terminal.
    clear_terminal();

    // Ask the user how many AI players they would like to play against.
    let number_of_ai = ask_amount();
    let players = Name::get_random(number_of_ai).unwrap();

    // Ask the user if they would like to see the rules of poker.
    let show_rules = ask_show_rules();
    if show_rules {
        clear_terminal();
        read_rules_file();
    }

    // todo: any key to continue...

    for player in players {
        println!("{} has been chosen.", player);
    }
}
