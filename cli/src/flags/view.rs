use clap::Parser;

#[derive(Debug, Parser)]
pub struct ViewFlags {
    /// File to view it's content
    #[clap(short, long, value_name = "FILE")]
    pub file: String,
}
