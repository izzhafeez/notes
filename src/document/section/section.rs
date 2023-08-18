use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use serde::{Serialize};

use crate::document::information::information::Information;

#[derive(Serialize)]
pub struct Section {
    header: String,
    information: HashMap<String, Vec<String>>,
    related: HashMap<String, Vec<String>>
}

impl Section {
    pub fn parse(s: &str) -> Result<Self, ()> {
        let raw_lines: Vec<&str> = s.split("\n").collect();
        let mut lines_iter = raw_lines.into_iter();
        let header: String = lines_iter
            .next()
            .unwrap()
            .to_string();

        let mut information: HashMap<String, Vec<String>> = HashMap::new();
        let mut related: HashMap<String, Vec<String>> = HashMap::new();
        let information_vec = lines_iter
            .skip(0)
            .map(Information::parse)
            .map(|i| i.unwrap());

        for i in information_vec {
            let category: String = i.get_category();
            let text: String = i.get_text();
            match category.as_str() {
                "Ancestors" | "Children" | "Related" => {
                    match related.get_mut(&category) {
                        Some(vec) => {
                            vec.push(text);
                        },
                        None => {
                            related.insert(category, vec![text]);
                        }
                    }
                },
                _ => {
                    match information.get_mut(&category) {
                        Some(vec) => {
                            vec.push(text);
                        },
                        None => {
                            information.insert(category, vec![text]);
                        }
                    }
                }
            };
        }
        Ok(Self { header, information, related })
    }

    pub fn get_header(&self) -> String {
        self.header.to_string()
    }

    pub fn update_related(section: Section, headers_set: &HashSet<String>, headers_vec: &Vec<String>) -> Self {
        let header: String = section.header;
        let information: HashMap<String, Vec<String>> = section.information;
        let related: HashMap<String, Vec<String>> = section.related
            .into_iter()
            .map(|e| Section::process_related_entry(e, headers_set, headers_vec))
            .collect();
        Self { header, information, related }
    }

    fn process_related_entry((k, v): (String, Vec<String>), headers_set: &HashSet<String>, headers_vec: &Vec<String>) -> (String, Vec<String>) {
        (k, v.into_iter()
            .map(|s| Section::process_related_string(s, headers_set, headers_vec))
            .collect())
    }

    fn process_related_string(s: String, headers_set: &HashSet<String>, headers_vec: &Vec<String>) -> String {
        match headers_set.contains(&s) {
            true => s,
            false => Section::find_matching_header(s.clone(), headers_vec)
        }
    }

    fn find_matching_header(s: String, headers_vec: &Vec<String>) -> String {
        headers_vec
            .into_iter()
            .filter(|h| h.starts_with(&s))
            .next()
            .unwrap()
            .clone()
    }
}

impl Display for Section {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.header, &self.related
            .iter()
            .map(|(k, v)| format!("{}: {:?}", k, v))
            .collect::<Vec<String>>()
            .join("\n")
        )
    }
}