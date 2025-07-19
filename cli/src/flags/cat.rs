use clap::Parser;

#[derive(Debug, Parser)]
pub struct CatFlags {
    /// Path to get the file content
    #[clap(short, long, value_name = "PATH", default_value = ".")]
    pub path: Option<String>,

    /// Number of lines
    #[clap(short, long)]
    pub number: Option<u32>,
}
