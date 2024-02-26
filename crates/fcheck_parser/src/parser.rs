use crate::text::parse_contents_to_module;
use fcheck_diagnostic::Diagnostic;
use fcheck_io::read_from_filepath;
use rustpython_parser::ast::Mod;
use rustpython_parser::ParseError;

/// Parses a given module object and returns a list of Issues.
/// We take in the ` (meaning it at least had a valid AST and such) and returns a list
fn parse_module(_module: &Mod) -> Vec<Diagnostic> {
    let errors: Vec<Diagnostic> = Vec::new();
    // TODO: Implement!
    errors
}

/// Parses a given file path and outputs issues.
pub fn get_issues_for_file_at_filepath(filepath: &str) -> Result<Vec<Diagnostic>, ParseError> {
    let contents = read_from_filepath(filepath);
    let module = parse_contents_to_module(&contents)?;
    Ok(parse_module(&module))
}
