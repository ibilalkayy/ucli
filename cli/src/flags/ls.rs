use clap::Parser;

#[derive(Debug, Parser)]
pub struct ListFlags {
    /// Directory path to list the content
    #[clap(short, long, value_name = "PATH", default_value = ".")]
    pub path: String,

    /// Show all the hidden files
    #[clap(long)]
    pub all: bool,

    /// Long listing format
    #[clap(long)]
    pub long: bool,
}
