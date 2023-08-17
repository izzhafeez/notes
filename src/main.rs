use crate::document::document::Document;
use crate::reader::directory_reader::loop_directory;
use crate::reader::file_reader::{read_all_files, read_file};

mod document;
mod reader;

fn main() -> Result<(), ()> {
    let files = loop_directory()?;
    let documents = files
        .into_iter()
        .map(|(k, v)| v)
        .map(read_all_files)
        .map(|v| v
            .into_iter()
            .map(Document::parse)
            .map(|s| s.unwrap().to_string())
            .collect::<Vec<String>>()
            .join("\n\n\n"))
        .collect::<Vec<String>>()
        .join("\n\n\n\n");

    println!("{}", documents);

    Ok(())
}
