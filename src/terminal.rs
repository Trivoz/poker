//! This file contains functions that pertain to the terminal.
//!
//! Anything that directly involves manipulating the functionality of the terminal
//! or text in relation to the terminal such as centering, formatting, etc.

use std::process::Command;

/// Clear the terminal.
///
/// # Panics
///
/// * If the terminal cannot be cleared, in this case, if the `clear` command is not found.
pub fn clear_terminal() {
    Command::new("clear")
        .status()
        .expect("Unable to clear terminal, please clear manually");
}
