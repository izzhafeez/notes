pub fn get_category(c: char) -> Result<String, ()> {
    match c {
        'A' => Ok(String::from("Ancestors")),
        'B' => Ok(String::from("Benefits")),
        'C' => Ok(String::from("Children")),
        'D' => Ok(String::from("Definitions")),
        'E' => Ok(String::from("Explanations")),
        'N' => Ok(String::from("Negatives")),
        'R' => Ok(String::from("Related")),
        'T' => Ok(String::from("Properties")),
        'U' => Ok(String::from("Purposes")),
        'X' => Ok(String::from("Examples")),
        _ => Err(())
    }
}