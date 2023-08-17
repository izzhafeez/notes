use std::fmt::{Display, Formatter};

pub struct Header {
    text: String
}

impl Header {
    pub fn new(s: &str) -> Self {
        Header { text: s.replace("\\\\", "").parse().unwrap() }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}