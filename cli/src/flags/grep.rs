use clap::Parser;

#[derive(Debug, Parser)]
pub struct GrepFlags {
    /// The pattern to search for
    #[clap(short, long)]
    pub pattern: String,

    /// Case insensitive
    #[clap(long)]
    pub case_insensitive: bool,

    /// Invert match
    #[clap(long)]
    pub invert: bool,

    /// Show the line numbers
    #[clap(long)]
    pub numbers: bool,
}
