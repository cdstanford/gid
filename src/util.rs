/*
    Utility
*/

use serde::de::DeserializeOwned;
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn path_reader<P: AsRef<Path>>(path: P) -> BufReader<File> {
    BufReader::new(File::open(path).unwrap())
}

pub fn from_json_file<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> T {
    from_reader(path_reader(path)).unwrap()
}

// TODO
// https://stackoverflow.com/questions/22355273/writing-to-a-file-or-stdout-in-rust
// Path writer or stdout if not provided
// fn path_writer<P: AsRef<Path>>(path: P) -> impl Write {
//     unimplemented!()
// }
