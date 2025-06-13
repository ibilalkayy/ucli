use clap::Parser;

#[derive(Debug, Parser)]
pub struct RenderFlags {
    /// Give the input file
    #[clap(short, long, value_name = "FILE")]
    input: String,

    /// Output file (optional)
    #[clap(short, long, value_name = "FILE")]
    output: Option<String>,

    /// Output format (default: html)
    #[clap(short, long, value_name = "html | plaintext | markdown")]
    format: String,

    /// Apply a specific template (if supported)
    #[clap(short, long, value_name = "TEMPLATE")]
    template: String,

    /// Output raw HTML without styling
    #[clap(long)]
    no_style: bool,
}
