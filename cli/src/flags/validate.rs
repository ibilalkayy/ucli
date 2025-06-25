use clap::Parser;

#[derive(Debug, Parser)]
pub struct ValidateFlags {
    /// File to validate
    #[clap(short, long, value_name = "FILE")]
    pub file: String,

    /// Setup the stricter validation
    #[clap(long)]
    pub strict: bool,

    /// Show details of checks
    #[clap(long)]
    pub verbose: bool,
}
