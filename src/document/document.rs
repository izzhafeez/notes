#![crate_name = "doc"]

use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use serde::{Serialize};

use crate::document::section::section::Section;

/// A document captures the contents of a tex file.
#[derive(Serialize)]
pub struct Document {
    /// A hashmap of section headings to sections.
    sections: HashMap<String, Section>
}

impl Document {
    /// Reads the contents of a tex file and processes them into a Document.
    ///
    /// # Arguments
    ///
    /// * `s` - A string representing the contents of a tex file.
    pub fn parse(s: String) -> Result<Self, ()> {
        let raw_sections: Vec<&str> = s
            .split("\n\n")
            .skip(1)
            .filter(|section| !section.contains("\\end"))
            .collect();
        let raw_sections: HashMap<String, Section> = raw_sections
            .into_iter()
            .map(Section::parse)
            .map(|section| section.unwrap())
            .map(|section| (section.get_header(), section))
            .collect();
        let sections: HashMap<String, Section> = Document::join_sections(raw_sections);
        Ok(Self { sections })
    }

    /// Joins sections based on their Ancestors, Children and Related sections.
    ///
    /// # Arguments
    ///
    /// * `sections` - A hashmap of section headings to sections.
    ///
    /// # Returns
    ///
    /// * An updated hashmap of section headings to sections.
    ///
    /// # Examples
    ///
    /// For example, if section "Identity Matrix" has ancestor section "Matrix",
    /// then it will join them together.
    fn join_sections(sections: HashMap<String, Section>) -> HashMap<String, Section> {
        let headers_set: HashSet<String> = sections
            .iter()
            .map(|(k, _v)| k.to_string())
            .collect();
        let headers_vec: Vec<String> = sections
            .iter()
            .map(|(k, _v)| k.to_string())
            .collect();
        sections
            .into_iter()
            .map(|(k, section)| (k, Section::update_related(section, &headers_set, &headers_vec)))
            .collect()
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.sections
            .iter()
            .map(|(_k, v)| v.to_string())
            .collect::<Vec<String>>()
            .join("\n\n"))
    }
}