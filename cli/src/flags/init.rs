use clap::Parser;

#[derive(Debug, Parser)]
pub struct InitFlags {
    /// Path to create the file
    #[clap(short, long, value_name = "PATH", default_value = ".")]
    pub path: Option<String>,

    /// Overwrite if file exists
    #[clap(long)]
    pub force: bool,
}
