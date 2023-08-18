use std::fs;
use std::path::PathBuf;

pub fn read_file(path: PathBuf) -> String {
    fs::read_to_string(path).expect("No such file")
}

pub fn read_all_files(paths: Vec<PathBuf>) -> Vec<String> {
    paths
        .into_iter()
        .map(read_file)
        .collect::<Vec<String>>()
}