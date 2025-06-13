use clap::Parser;

#[derive(Debug, Parser)]
pub struct ValidateFlags {
    /// File to validate
    #[clap(short, long, value_name = "FILE")]
    input: String,

    /// Output errors as JSON (useful in CI)
    #[clap(long)]
    json: bool,

    /// Enable strict validation
    #[clap(long)]
    strict: bool,
}
