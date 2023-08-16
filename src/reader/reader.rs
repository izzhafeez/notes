use std::fs;

pub fn loop_directory() {
    let paths = fs::read_dir("./data")
        .unwrap()
        .map(|path| path.unwrap().path())
        .filter(|path| !path.ends_with(".DS_Store"));

    for path in paths {
        println!("{}", path.display());
        let subpaths = fs::read_dir(&path)
            .unwrap()
            .map(|path| path.unwrap().path());
        for subpath in subpaths {
            println!("{}", subpath.display());
        }
    }
}