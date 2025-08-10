use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct ListFlags {
    /// Directory path to list the files
    #[clap(short, long, value_name = "PATH", default_value = ".")]
    pub path: Option<PathBuf>,

    /// Show all the hidden files
    #[clap(short, long)]
    pub all: bool,

    /// Long listing format of files
    #[clap(short, long)]
    pub long: bool,
}
