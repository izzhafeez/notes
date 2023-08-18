use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use crate::document::document::Document;

pub fn write_json(documents: HashMap<String, Vec<Document>>) -> Result<(), ()> {
    let mut file: File = File::create("./processed/notes.json").unwrap();
    let json_string: String = serde_json::to_string(&documents).unwrap();
    file.write(json_string.as_ref()).expect("Write failed.");
    println!("{:?}", json_string);

    Ok(())
}