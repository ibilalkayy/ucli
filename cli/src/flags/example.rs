use clap::Parser;

#[derive(Debug, Parser)]
pub struct ExampleFlags {
    /// Output path
    #[clap(short, long, value_name = "PATH")]
    pub path: String,
}
