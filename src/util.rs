/*
    System utility functions

    (File I/O, JSON serialization, system time, etc.)
*/

use chrono::offset::Local;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/*
    File I/O
*/

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

pub fn file_exists<P>(path: P) -> bool
where
    P: AsRef<Path> + Debug,
{
    path.as_ref().exists()
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
    Path walking functions
*/

// Walk files in a directory
// Could also just use the walkdir crate for this.
// It is truly hilarious how many layers of indirection it takes
// to go through Rust's various String and OS abstractions
pub fn paths_in(dir: &str) -> impl Iterator<Item = String> + '_ {
    fs::read_dir(PathBuf::from(dir))
        .unwrap_or_else(|err| {
            panic!("couldn't view files in directory: {} ({})", dir, err)
        })
        .map(move |file| {
            file.unwrap_or_else(|err| {
                panic!("error viewing file in directory: {} ({})", dir, err)
            })
        })
        .map(|file| file.path().into_os_string())
        .map(|osstr| {
            osstr.into_string().unwrap_or_else(|err| {
                panic!("found file path with invalid unicode ({:?})", err)
            })
        })
}

// Recursively visit all directories in a directory,
// calling the closure cb
// https://doc.rust-lang.org/std/fs/fn.read_dir.html#examples
pub fn walk_dirs_rec<F: Fn(&Path)>(dir: &Path, cb: &F) -> io::Result<()> {
    if dir.is_dir() {
        cb(dir);
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            walk_dirs_rec(&path, cb)?;
        }
    }
    Ok(())
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
