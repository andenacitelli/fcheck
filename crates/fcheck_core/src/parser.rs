use crate::disk::read_from_filepath;
use crate::model::Issue;
use log::error;
use rustpython_parser::ast::Mod;
use rustpython_parser::{parse, Mode};
use std::process::exit;

fn parse_contents_to_module(filepath: &str, contents: &str) -> Mod {
    match parse(contents, Mode::Module, "<embedded>") {
        Ok(module) => module,
        Err(error) => {
            // TODO: Would be nicer for this not to be completely fatal.
            error!(
                "Syntax Error in file {filepath} prevented further checking: {error}",
                filepath = filepath,
                error = error.error
            );
            exit(1);
        }
    }
}

/// Parses a given module object and returns a list of Issues.
/// We take in the ` (meaning it at least had a valid AST and such) and returns a list
fn parse_module(_module: &Mod) -> Vec<Issue> {
    let errors: Vec<Issue> = Vec::new();
    // TODO: Implement!
    errors
}

/// Parses a given file path and outputs issues.
pub fn get_issues_for_file_at_filepath(filepath: &str) -> Vec<Issue> {
    let contents = read_from_filepath(filepath);
    let module = parse_contents_to_module(filepath, &contents);
    parse_module(&module)
}
