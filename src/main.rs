use crate::reader::directory_reader::loop_directory;
use crate::reader::file_reader::read_file;

mod document;
mod reader;

fn main() -> Result<(), ()> {
    let files = loop_directory()?;
    for (k, v) in files {
        println!("{} {:?}", k, v.into_iter().map(read_file).collect::<Vec<String>>());
    }

    Ok(())
}
