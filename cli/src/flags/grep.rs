use clap::Parser;

#[derive(Debug, Parser)]
pub struct GrepFlags {
    /// File to search
    #[clap()]
    pub file: String,

    /// The pattern to search for
    #[clap(short, long)]
    pub pattern: String,

    /// Match the case insensitive pattern
    #[clap(short, long)]
    pub case_insensitive: bool,

    /// Show lines that don't match the pattern
    #[clap(short, long)]
    pub invert: bool,

    /// Show the line numbers in the output
    #[clap(short, long)]
    pub number: bool,
}
