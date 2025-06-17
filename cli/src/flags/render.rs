use clap::Parser;
use render::render::Format;

#[derive(Debug, Parser)]
pub struct RenderFlags {
    /// Give the input file
    #[clap(short, long, value_name = "FILE")]
    pub input: String,

    /// Output file (optional)
    #[clap(short, long, value_name = "FILE")]
    pub output: Option<String>,

    /// Output format
    #[clap(
        short,
        long,
        default_value = "json",
        value_name = "json | html | plaintext | markdown"
    )]
    pub format: Format,

    /// Apply a specific template (if supported)
    #[clap(short, long, value_name = "TEMPLATE")]
    pub template: String,

    /// Output raw HTML without styling
    #[clap(long)]
    pub no_style: bool,
}
