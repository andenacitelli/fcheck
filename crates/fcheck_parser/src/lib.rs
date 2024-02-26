// This is the only function we expose externally. Given a file path, return a list of issues with it.
pub use parser::get_issues_for_file_at_filepath;
mod parser;

// Definitions of other internal modules within this crate.
mod text;
