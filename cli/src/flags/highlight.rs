use clap::Parser;

#[derive(Debug, Parser)]
pub struct HighlightFlags {
    /// File to validate
    #[clap(short, long, value_name = "FILE")]
    pub file: String,

    /// Optional color theme
    #[clap(short, long, value_name = "THEME")]
    pub theme: Option<String>,
}
