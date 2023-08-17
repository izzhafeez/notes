use std::fmt::{Display, Formatter};

use crate::document::header::header::Header;
use crate::document::information::information::Information;

pub struct Section {
    header: Header,
    information: Vec<Information>
}

impl Section {
    pub fn parse(s: &str) -> Result<Self, ()> {
        let raw_lines: Vec<&str> = s.split("\n").collect();
        let mut lines_iter = raw_lines.into_iter();
        let header: Header = lines_iter
            .next()
            .map(Header::new)
            .unwrap();
        let information: Vec<Information> = lines_iter
            .skip(0)
            .map(Information::new)
            .collect();
        Ok(Self { header, information })
    }
}

impl Display for Section {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.header, &self.information
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("\n")
        )
    }
}