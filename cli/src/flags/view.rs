use clap::Parser;

#[derive(Debug, Parser)]
pub struct ViewFlags {
    /// File to view it's content
    #[clap(short, long, value_name = "FILE")]
    pub file: Option<String>,

    /// Number of lines per page
    #[clap(short, long, default_value = "20")]
    pub lines: usize,
}
