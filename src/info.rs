//! This module contains anything pertaining to reading the rules,
//! or helping the player in the game of poker. Things such as opening
//! and reading the rules of poker and so on might be found in this file.
//!
//! File description: To be determined...

/// Read the rules of poker from a file.
///
/// # Panics
///
/// * If the file cannot be found.
pub fn read_rules_file() {
    let rules = std::fs::read_to_string("rules.txt").expect("Unable to find rules.txt");
    println!("{}", rules);
}
