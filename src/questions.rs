//! It just so happens that sometimes we have to ask questions to the user.
//! This is normally to customize the game to the players needs.
//!
//! This file contains some of the functions that pertain to asking these sorts of questions.
//!
//! todo: tests for this file

use super::{MAX_PLAYERS, MIN_PLAYERS};

/// Ask the user how many AI players they would like to play against.
///
/// # Returns
///
/// * `u8` - A selection of how many AI players to play against between 1 and 4.
///
/// # Panics
///
/// * If the user does not enter a number between 1 and 4.
pub fn ask_amount() -> u8 {
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
pub fn ask_show_rules() -> bool {
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
