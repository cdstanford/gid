/*
    Utility for file I/O and JSON serialization
*/

use chrono::offset::Local;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::time::{Duration, SystemTime};

fn path_reader<P>(path: P) -> BufReader<File>
where
    P: AsRef<Path> + Debug,
{
    BufReader::new(File::open(&path).unwrap_or_else(|err| {
        panic!("Could not open file for reading: {:?} -- {}", path, err)
    }))
}

fn path_writer<P>(path: P) -> BufWriter<File>
where
    P: AsRef<Path> + Debug,
{
    BufWriter::new(File::create(&path).unwrap_or_else(|err| {
        panic!("Could not open file for writing: {:?} -- {}", path, err)
    }))
}

pub fn from_json_file<P, T>(path: P) -> T
where
    P: AsRef<Path> + Debug,
    T: DeserializeOwned,
{
    serde_json::from_reader(path_reader(&path)).unwrap_or_else(|err| {
        panic!("Could not read JSON from {:?} -- {}", path, err)
    })
}

pub fn to_json_file<P, T>(path: P, data: T)
where
    P: AsRef<Path> + Debug,
    T: Serialize,
{
    let mut writer = path_writer(&path);
    serde_json::to_writer_pretty(&mut writer, &data).unwrap_or_else(|err| {
        panic!("Could not write JSON to {:?} -- {}", path, err)
    });
    writeln!(&mut writer).unwrap_or_else(|err| {
        format!("Could not append newline to file: {:?} -- {}", &path, err);
    });
}

pub fn lines_to_file<P>(path: P, lines: Vec<String>)
where
    P: AsRef<Path> + Debug,
{
    let mut writer = path_writer(&path);
    for line in &lines {
        writeln!(writer, "{}", line).unwrap();
    }
}

/*
    Time-related functions
*/

// Time elapsed with error message
pub fn time_since(t: &SystemTime) -> Duration {
    t.elapsed().unwrap_or_else(|err| {
        panic!(
            "Getting system time elapsed failed (was system clock reset?): {}",
            err
        )
    })
}

// Current datetime for use in file names
pub fn current_datetime_str() -> String {
    Local::now().format("%Y-%m-%d-%H%M%S").to_string()
}
