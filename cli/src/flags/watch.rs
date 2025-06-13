use clap::Parser;

#[derive(Debug, Parser)]
pub struct WatchFlags {
    /// File to watch
    #[clap(short, long, value_name = "PATH")]
    path: String,

    /// Run this command on change (lint, render)
    #[clap(short = 'c', long, value_name = "COMMAND")]
    on_change: String,

    /// Output format (json, html, etc)
    #[clap(short, long, value_name = "FORMAT", default_value = "json")]
    format: String,

    /// Output path for the generated file
    #[clap(short, long, value_name = "OUTPUT")]
    output: Option<String>,
}
