use clap::Parser;

#[derive(Debug, Parser)]
pub struct ParseFlags {
    /// Give the input file
    #[clap(short, long, value_name = "FILE")]
    input: String,

    /// Output file (optional)
    #[clap(short, long, value_name = "FILE")]
    output: Option<String>,

    /// Output format (default: json)
    #[clap(short, long, value_name = "json | debug")]
    format: String,

    /// Treat unknown syntax as an error
    #[clap(long)]
    strict: bool,

    /// Pretty-print the output
    #[clap(long)]
    pretty: bool,
}
