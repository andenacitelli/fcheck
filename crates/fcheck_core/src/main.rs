mod args;
mod config;
mod parser;

use dotenv::dotenv;
use fcheck_parser::get_issues_for_file_at_filepath;
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
    for diagnostic in get_issues_for_file_at_filepath(filepath) {
        error!(
            "[{kind}, {filepath}, {line}:{column}]: {message}",
            kind = diagnostic.kind.name,
            filepath = filepath,
            line = diagnostic.location.line,
            column = diagnostic.location.column,
            message = diagnostic.kind.body
        )
    }
}

fn main() {
    dotenv().ok(); // load .env
    config::validate(); // statically validate environment variables
    env_logger::init(); // env_logger is contextual on RUST_LOG env var

    let args = args::parse();
    println!("Processing Files {:?}", args.files);

    // Actually run validations
    for path in get_file_paths_from_args() {
        debug!("Triggering check on file path {}", path);
        handle_filepath(&path);
    }
}
