use clap::Parser;

#[derive(Debug, Parser)]
pub struct ValidateFlags {
    /// File to validate
    #[clap(short, long, value_name = "FILE")]
    pub input: String,

    /// Output errors as JSON (useful in CI)
    #[clap(long)]
    pub json: bool,

    /// Enable strict validation
    #[clap(long)]
    pub strict: bool,
}
