use crate::flags::{
    init::InitFlags, lint::LintFlags, parse::ParseFlags, render::RenderFlags,
    validate::ValidateFlags, watch::WatchFlags,
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
    /// Generate a starter file
    Init(InitFlags),

    /// Parse and print DSL
    Parse(ParseFlags),

    /// Render DSL to HTML or other files
    Render(RenderFlags),

    /// Check if DSL file has a valid syntax
    Validate(ValidateFlags),

    /// Watch the file
    Watch(WatchFlags),

    /// Analyze the file data to detect errors
    Lint(LintFlags),
}
