use clap::Parser;

#[derive(Debug, Parser)]
pub struct SortFlags {
    /// File to sort
    #[clap(short, long, value_name = "FILE")]
    pub file: String,

    /// Reverse order
    #[clap(long)]
    pub reverse: bool,

    /// Numeric sorting
    #[clap(long)]
    pub number: bool,
}
