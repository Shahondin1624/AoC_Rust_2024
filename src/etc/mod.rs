pub mod solution;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_input(filename: &str) -> String {
    let mut path = Path::new("input").join(filename);
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}
