use clap::Parser;

#[derive(Debug, Parser)]
pub struct ParseFlags {
    /// Give the input file
    #[clap(short, long, value_name = "FILE")]
    pub input: String,

    // /// Output file (optional)
    // #[clap(short, long, value_name = "FILE")]
    // pub output: Option<String>,
    /// Output format (default: json)
    #[clap(short, long, value_name = "json | debug")]
    pub format: String,

    /// Treat unknown syntax as an error
    #[clap(long)]
    pub strict: bool,

    /// Pretty-print the output
    #[clap(long)]
    pub pretty: bool,
}
