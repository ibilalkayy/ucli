use crate::flags::{
    convert::ConvertFlags, example::ExampleFlags, highlight::HighlightFlags, init::InitFlags,
    parse::ParseFlags, validate::ValidateFlags,
};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    author = "Bilal Khan",
    version,
    about = "DSL is a Rust-based CLI tool for parsing, validating, and rendering your own custom domain-specific language."
)]
pub struct Dsl {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Creates a new file with a basic template to help you get started.
    Init(InitFlags),

    /// Checks the file for errors or incorrect syntax.
    Validate(ValidateFlags),

    /// Parses the file and displays its underlying structure.
    Parse(ParseFlags),

    /// Converts the file into another format (e.g. HTML, JSON).
    Convert(ConvertFlags),

    /// Displays the file in color for easier reading in the terminal.
    Highlight(HighlightFlags),

    /// Generates a demo file to quickly see how the language looks.
    Example(ExampleFlags),

    Rules,
}
