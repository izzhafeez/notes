pub struct Header {
    text: String
}

impl Header {
    pub fn new(s: &str) -> Self {
        Header { text: s.parse().unwrap() }
    }
}