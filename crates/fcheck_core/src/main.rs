#![forbid(unsafe_code)]
#![warn(unused_imports)]
#![warn(unused_doc_comments)]

use std::env;
use std::fs;

use log::{error, info};
use rustpython_parser::ast::Mod;
use rustpython_parser::{parse, Mode};

/// Parses a given rustpython_parser module, pointing out any issues.
fn handle_module(_module: &Mod) -> () {
    info!("Successfully parsed in module.");
}

/// Parses a given file path and outputs issues.
fn parse_contents(filepath: &str) -> () {
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    info!("Parsing:\n```python\n{}\n```", &contents);
    match parse(&contents, Mode::Module, "<embedded>") {
        Ok(module) => handle_module(&module),
        Err(e) => error!(
            "{filepath}: {error}",
            filepath = e.source_path,
            error = e.error
        ),
    };
}

/// Analyzes argv and returns a list of file paths we should run on.
fn get_file_path_from_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let mut paths: Vec<String> = Vec::new();

    let file_path = &args[1];
    paths.push(file_path.to_string());

    return paths;
}

fn main() {
    for path in get_file_path_from_args() {
        info!("Analyzing file path {filepath}", filepath = path);
        parse_contents(&path)
    }
}
