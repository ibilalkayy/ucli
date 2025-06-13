pub mod command;
pub mod flags;

use crate::command::command::{Command, Dsl};
use clap::Parser;

use domains::parses::{Format, Output, Template, parse_options};

fn main() {
    let dsl = Dsl::parse();
    match dsl.command {
        Command::Init(_) => todo!(),
        Command::Parse(p) => {
            parse_options(
                Output { file: p.input },
                Format {
                    json: false,
                    html: false,
                    plaintext: true,
                    markdown: false,
                },
                Template {
                    template_type: "hello template".to_string(),
                },
            );
        }
        Command::Render(_) => todo!(),
        Command::Validate(_) => todo!(),
        Command::Watch(_) => todo!(),
        Command::Lint(_) => todo!(),
    }
}
