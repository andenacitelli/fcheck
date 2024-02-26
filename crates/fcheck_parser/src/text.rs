/// Module contains parser utilities that relate directly to text.
use log::error;
use rustpython_parser::ast::Mod;
use rustpython_parser::{parse, Mode};
use std::process::exit;

/// Parses the given text block into a Python AST.
/// Raises an error if the text block is not valid Python, in which circumstance we cannot parse further.
/// This is a raised error rather than a full panic or a program exit because we still want to enable parsing other files even if this one has non-bypassable problems.
pub fn parse_contents_to_module(contents: &str) -> Mod {
    match parse(contents, Mode::Module, "<embedded>") {
        Ok(module) => module,
        Err(error) => {
            // TODO: Would be nicer for this not to be completely fatal.
            error!(
                "Syntax Error prevented further checking: {error}",
                error = error.error
            );
            exit(1);
        }
    }
}
