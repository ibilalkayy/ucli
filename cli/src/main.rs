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
    Init,

    /// Parse and print DSL
    Parse,

    /// Render DSL to HTML or other files
    Render,

    /// Check if DSL file has valid syntax
    Validate,
}

fn main() {
    let dsl = Dsl::parse();
    match dsl.command {
        Command::Init => println!("initialized"),
        Command::Parse => println!("parsed"),
        Command::Render => println!("rendered"),
        Command::Validate => println!("validated"),
    }
}
