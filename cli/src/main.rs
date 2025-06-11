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
}

#[derive(Debug, Parser)]
pub struct InitFlags {
    /// Create a new project in this directory
    #[clap(short, long, value_name = "DIR", default_value = ".")]
    dir: String,

    /// Example template to use
    #[clap(short, long, value_name = "TEMPLATE", default_value = "basic")]
    template: String,

    /// Overwrite existing files if present
    #[clap(long)]
    force: bool,
}

#[derive(Debug, Parser)]
pub struct ParseFlags {
    /// Give the input file
    #[clap(short, long, value_name = "FILE")]
    input: String,

    /// Output file (optional)
    #[clap(short, long, value_name = "FILE")]
    output: Option<String>,

    /// Output format (default: json)
    #[clap(short, long, value_name = "json | debug")]
    format: String,

    /// Treat unknown syntax as an error
    #[clap(long)]
    strict: bool,

    /// Pretty-print the output
    #[clap(long)]
    pretty: bool,
}

#[derive(Debug, Parser)]
pub struct RenderFlags {
    /// Give the input file
    #[clap(short, long, value_name = "FILE")]
    input: String,

    /// Output file (optional)
    #[clap(short, long, value_name = "FILE")]
    output: Option<String>,

    /// Output format (default: html)
    #[clap(short, long, value_name = "html | plaintext | markdown")]
    format: String,

    /// Apply a specific template (if supported)
    #[clap(short, long, value_name = "TEMPLATE")]
    template: String,

    /// Output raw HTML without styling
    #[clap(long)]
    no_style: bool,
}

#[derive(Debug, Parser)]
pub struct ValidateFlags {
    /// File to validate
    #[clap(short, long, value_name = "FILE")]
    input: String,

    /// Output errors as JSON (useful in CI)
    #[clap(long)]
    json: bool,

    /// Enable strict validation
    #[clap(long)]
    strict: bool,
}

fn main() {
    let dsl = Dsl::parse();
    match dsl.command {
        Command::Init(_) => todo!(),
        Command::Parse(_) => todo!(),
        Command::Render(_) => todo!(),
        Command::Validate(_) => todo!(),
    }
}
