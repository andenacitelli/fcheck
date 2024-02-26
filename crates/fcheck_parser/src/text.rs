/// Module contains parser utilities that relate directly to text.
use rustpython_parser::ast::Mod;
use rustpython_parser::{parse, Mode, ParseError};

/// Parses the given text block into a Python AST.
/// Raises an error if the text block is not valid Python, in which circumstance we cannot parse further.
/// This is a raised error rather than a full panic or a program exit because we still want to enable parsing other files even if this one has non-bypassable problems.
pub fn parse_contents_to_module(contents: &str) -> Result<Mod, ParseError> {
    parse(contents, Mode::Module, "<embedded>")
}
