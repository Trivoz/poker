//!    Copyright 2023 Joshua Rose
//!
//!Licensed under the Apache License, Version 2.0 (the "License");
//!you may not use this file except in compliance with the License.
//!You may obtain a copy of the License at
//!
//!  <http://www.apache.org/licenses/LICENSE-2.0>
//!
//!Unless required by applicable law or agreed to in writing, software
//!distributed under the License is distributed on an "AS IS" BASIS,
//!WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//!See the License for the specific language governing permissions and
//!limitations under the License.
//!
//! # poker_rs
//!
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

mod ai;
mod cards;
mod info;
mod questions;
mod terminal;

use ai::name::Name;
use info::read_rules_file;
use questions::{ask_amount, ask_show_rules};
use rand::Rng;
use terminal::clear_terminal;

/// The minimum amount of players in a game of poker.
const MIN_PLAYERS: u8 = 1;

/// Whether or not to show the rules of poker by default.
const SHOW_RULES_AS_DEFAULT: bool = false;

/// The default amount of players if an invalid number is supplied.
const DEFAULT_PLAYERS: u8 = 2;

/// The maximum amount of players in a game of poker.
const MAX_PLAYERS: u8 = 4;

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
