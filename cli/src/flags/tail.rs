use clap::Parser;

#[derive(Debug, Parser)]
pub struct TailFlags {
    /// Show the last line of a file
    #[clap(short, long, value_name = "FILE")]
    pub file: String,

    /// Number of lines to show
    #[clap(long)]
    pub number: bool,

    /// Follow file (watch for changes)
    #[clap(long)]
    pub follow: bool,
}
