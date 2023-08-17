use std::fmt::{Display, Formatter};

use crate::document::section::section::Section;

pub struct Document {
    sections: Vec<Section>
}

impl Document {
    pub fn parse(s: String) -> Result<Self, ()> {
        let raw_sections: Vec<&str> = s
            .split("\n\n")
            .skip(1)
            .collect();
        let sections: Vec<Section> = raw_sections
            .into_iter()
            .map(Section::parse)
            .map(|section| section.unwrap())
            .collect();
        Ok(Self { sections })
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.sections
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n\n"))
    }
}