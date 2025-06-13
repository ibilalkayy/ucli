use clap::Parser;

#[derive(Debug, Parser)]
pub struct LintFlags {
    /// DSL file to lint
    #[clap(short, long, value_name = "FILE")]
    input: String,

    /// Enable on strict mode (fail on unknow syntax)
    #[clap(long)]
    strict: bool,

    /// Output as JSON (e.g., for editors/tools)
    #[clap(long)]
    json: bool,
}
