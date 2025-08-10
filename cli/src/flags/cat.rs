use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct CatFlags {
    /// Path to get the file content
    #[clap(short, long, value_name = "PATH", default_value = ".")]
    pub path: Option<PathBuf>,

    /// Line numbers to be added
    #[clap(short, long)]
    pub number: bool,
}
