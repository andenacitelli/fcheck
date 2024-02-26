use clap::Parser;
use std::path::PathBuf;

pub fn parse() -> Cli {
    Cli::parse()
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(required = true)]
    pub files: Vec<PathBuf>,
}
