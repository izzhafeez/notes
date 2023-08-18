use std::fmt::{Display, Formatter};
use serde::{Serialize};
use crate::document::information::category::category::get_category;

/// A piece of information, that can be in any one of many categories.
#[derive(Serialize)]
pub struct Information {
    category: String,
    text: String
}

impl Information {
    /// Parses a single line of text into a piece of Information.
    ///
    /// # Arguments
    ///
    /// * `s` - A string to be parsed.
    ///
    /// # Examples
    ///
    /// ```
    /// Information::parse("A: Matrix");
    /// Information::parse("R: Triangle Matrix");
    /// ```
    pub fn parse(s: &str) -> Result<Self, ()> {
        let text: String = s[3..].parse().unwrap();
        let first_letter = s
            .chars()
            .nth(0)
            .expect("Line is blank!");
        let category: String = get_category(first_letter)
            .expect(&format!("Invalid character token: {first_letter}"));
        Ok(Self { category, text })
    }

    pub fn get_category(&self) -> String {
        self.category.to_string()
    }

    pub fn get_text(&self) -> String {
        self.text.to_string()
    }
}

impl Display for Information {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}