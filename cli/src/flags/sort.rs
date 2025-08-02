use clap::Parser;

#[derive(Debug, Parser)]
pub struct SortFlags {
    /// File to sort
    #[clap(short, long, value_name = "FILE", default_value = ".")]
    pub file: Option<String>,

    /// Reverse order
    #[clap(short, long)]
    pub reverse: bool,

    /// Numeric sorting
    #[clap(short, long)]
    pub number: bool,
}
