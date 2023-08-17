use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn loop_directory() -> Result<HashMap<String, Vec<PathBuf>>, ()> {
    let paths = ls(PathBuf::from("./data"));

    let files: HashMap<String, Vec<PathBuf>> = paths
        .into_iter()
        .map(get_entries_from_dir)
        .collect();

    Ok(files)
}

fn ls(path: PathBuf) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .map(|path| path.unwrap().path())
        .filter(|path| !path.ends_with(".DS_Store"))
        .collect()
}

fn get_entries_from_dir(dir: PathBuf) -> (String, Vec<PathBuf>) {
    let dir_str: String = dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    (dir_str, ls(dir)
        .into_iter()
        .map(|p| p.join("Notes.tex"))
        .collect()
    )
}