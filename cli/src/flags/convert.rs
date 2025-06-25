use clap::Parser;
use convert::convert::TargetFormat;

#[derive(Debug, Parser)]
pub struct ConvertFlags {
    /// File to validate
    #[clap(short, long, value_name = "FILE")]
    pub file: String,

    /// Target format (html, json, toml...)
    #[clap(short, long, value_name = "FORMAT")]
    pub to: TargetFormat,

    /// Output file (optional)
    #[clap(short, long, value_name = "OUTPUT_FILE")]
    pub output: Option<String>,
}
