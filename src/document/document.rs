use crate::document::section::section::Section;

struct Document {
    sections: Vec<Section>
}

impl Document {
    fn parse(s: &str) -> Result<Self, ()> {
        let raw_sections: Vec<&str> = s
            .split("\n\n")
            .collect();
        let sections: Vec<Section> = raw_sections
            .into_iter()
            .map(Section::parse)
            .map(|section| section.unwrap())
            .collect();
        Ok(Self { sections })
    }
}