/// Class that we pipe all disk-based operations through in order to remain cohesive.
use std::fs;

pub fn read_from_filepath(filepath: &str) -> String {
    match fs::read_to_string(filepath) {
        Ok(contents) => contents,
        Err(err) => panic!(
            "Error reading filepath {filepath}: {err}",
            filepath = filepath,
            err = err
        ),
    }
}
