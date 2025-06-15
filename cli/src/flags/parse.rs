use clap::Parser;
use domains::parse::Format;

#[derive(Debug, Parser)]
pub struct ParseFlags {
    /// Give the input file
    #[clap(short, long, value_name = "FILE")]
    pub input: String,

    /// Output file (optional)
    #[clap(short, long, value_name = "FILE")]
    pub output: Option<String>,

    /// Give the output format
    #[clap(
        short,
        long,
        default_value = "json",
        value_name = "json | html | plaintext | markdown"
    )]
    pub format: Format,

    /// Treat unknown syntax as an error
    #[clap(long)]
    pub strict: bool,

    /// Pretty-print the output
    #[clap(long)]
    pub pretty: bool,
}
