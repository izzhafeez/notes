use std::fmt::{Display, Formatter};

use crate::document::information::parser::parser::{Parser, get, parse};

pub struct Information {
    text: String
}

impl Information {
    pub fn new(s: &str) -> Self {
        let text: String = s.parse().unwrap();
        Self { text }
    }

    fn get_parser(&self) -> Result<Parser, ()> {
        let first_letter = self.text
            .chars()
            .nth(0)
            .unwrap();
        get(first_letter)
    }

    fn parse(&self) -> Result<String, ()> {
        let parser: Parser = self.get_parser()?;
        parse(parser, &self.text)
    }
}

impl Display for Information {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}