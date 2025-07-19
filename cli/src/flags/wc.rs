use clap::Parser;

#[derive(Debug, Parser)]
pub struct WcFlags {
    /// Output path
    #[clap(short, long, value_name = "FILE")]
    pub file: String,

    /// Count lines
    #[clap(long)]
    pub lines: bool,

    /// Count words
    #[clap(long)]
    pub words: bool,

    /// Count bytes
    #[clap(long)]
    pub bytes: bool,
}
