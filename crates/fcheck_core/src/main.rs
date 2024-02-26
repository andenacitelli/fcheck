#![forbid(unsafe_code)]

mod config;
mod disk;
mod model;
mod parser;

use dotenv::dotenv;
use log::{debug, error};
use std::env;

/// Analyzes argv and returns a list of file paths we should run on.
fn get_file_paths_from_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let mut paths: Vec<String> = Vec::new();
    let file_path = &args[1];
    paths.push(file_path.to_string());
    paths
}

fn handle_filepath(filepath: &str) {
    for issue in parser::get_issues_for_file_at_filepath(filepath) {
        error!(
            "[{filepath}, lineno {lineno}]: {message}",
            filepath = issue.filepath,
            lineno = issue.line,
            message = issue.message
        )
    }
}

fn main() {
    // TODO: argv setup should happen here
    dotenv().ok(); // load .env
    config::validate(); // statically validate environment variables
    env_logger::init(); // env_logger is contextual on RUST_LOG env var

    // Actually run validations
    for path in get_file_paths_from_args() {
        debug!("Triggering check on file path {}", path);
        handle_filepath(&path);
    }
}

#[cfg(test)]
mod tests {
    use super::*; // import names from outer scope
    use log::info;

    #[test]
    fn basic_test() {
        let path = "../../samples/basic.py";
        info!("Opening file at {filepath}", filepath = path);
        handle_filepath(path)
    }
}
