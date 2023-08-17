use std::fs;
use std::path::PathBuf;

pub fn read_file(path: PathBuf) -> String {
    fs::read_to_string(path).expect("No such file")
}