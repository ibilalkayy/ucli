use clap::Parser;
use watch::watch::Format;

#[derive(Debug, Parser)]
pub struct WatchFlags {
    /// File to watch
    #[clap(short, long, value_name = "PATH")]
    pub path: String,

    /// Run this command on change (lint, render)
    #[clap(short = 'c', long, value_name = "COMMAND")]
    pub on_change: String,

    /// Give the output format
    #[clap(
        short,
        long,
        value_name = "json | html | plaintext | markdown",
        default_value = "json"
    )]
    pub format: Format,

    /// Output path for the generated file
    #[clap(short, long, value_name = "OUTPUT")]
    pub output: Option<String>,
}
