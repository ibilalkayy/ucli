use crate::flags::{
    cat::CatFlags, grep::GrepFlags, ls::ListFlags, sort::SortFlags, tail::TailFlags,
    view::ViewFlags, wc::WcFlags,
};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    author = "Bilal Khan",
    version,
    about = "A collection of minimalist Unix command-line tools again made in Rust."
)]
pub struct Younix {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Output contents of a file line-by-line
    Cat(CatFlags),

    /// List all the directory files
    Ls(ListFlags),

    /// Search for the matching lines
    Grep(GrepFlags),

    /// View the file content interactively
    View(ViewFlags),

    /// Sort the lines
    Sort(SortFlags),

    /// Last N lines of a file
    Tail(TailFlags),

    /// Count lines, words, bytes
    Wc(WcFlags),
}
