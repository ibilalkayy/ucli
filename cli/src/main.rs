pub mod command;
pub mod flags;

use crate::command::command::{Command, Dsl};
use clap::Parser;

use domains::parses::{Input, Output, parse_options};

fn main() {
    let dsl = Dsl::parse();
    match dsl.command {
        Command::Init(_) => todo!(),
        Command::Parse(p) => {
            parse_options(Input { file: p.input }, Output { file: p.output }, p.format);
        }
        Command::Render(_) => todo!(),
        Command::Validate(_) => todo!(),
        Command::Watch(_) => todo!(),
        Command::Lint(_) => todo!(),
    }
}
