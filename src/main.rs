use std::collections::HashMap;
use std::path::PathBuf;
use crate::document::document::Document;
use crate::reader::directory_reader::loop_directory;
use crate::reader::file_reader::{read_all_files};
use crate::writer::writer::write_json;

mod document;
mod reader;
mod writer;

fn main() -> Result<(), ()> {
    let paths_map: HashMap<String, Vec<PathBuf>> = loop_directory()?;
    let documents: HashMap<String, Vec<Document>> = paths_map
        .into_iter()
        .map(|(k, paths)| (k, read_all_files(paths)))
        .map(|(k, texts)| (k, texts
            .into_iter()
            .map(|text| Document::parse(text).unwrap())
            .collect()))
        .collect();

    write_json(documents).expect("Can't write.");

    Ok(())
}
