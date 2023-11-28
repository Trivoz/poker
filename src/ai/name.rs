//! Every AI has a name and the names are chosen randomly. Moreover, the names also
//! have properties such as default names, names that are better than others in relation
//! to poker skill, and so on.
//!
//! This file contains that sort of information.

extern crate strum;

use std::fmt;

use rand::seq::SliceRandom;

use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Debug, EnumIter)]
/// An enum of names for the AI players.
///
/// Note: This is a [`strum`] enum, and derives the [`EnumIter`] trait.
///
/// [`strum`]: https://docs.rs/strum/0.19.3/strum/
/// [`EnumIter`]: https://docs.rs/strum/0.19.3/strum/trait.EnumIter.html
pub enum Name {
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
    pub fn get_random(amount: u8) -> Result<Vec<Name>, &'static str> {
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
