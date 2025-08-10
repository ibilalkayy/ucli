use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct WcFlags {
    /// File path to target for counting
    #[clap(short, long, value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Count lines
    #[clap(short, long)]
    pub lines: bool,

    /// Count words
    #[clap(short, long)]
    pub words: bool,

    /// Count bytes
    #[clap(short, long)]
    pub bytes: bool,
}
