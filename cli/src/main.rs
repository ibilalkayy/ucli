pub mod command;
pub mod flags;

use crate::command::command::{Command, Dsl};
use clap::Parser;

use domains::{
    init::{InitData, init_options},
    parses::{Input, Output, parse_options},
};

fn main() {
    let dsl = Dsl::parse();
    match dsl.command {
        Command::Init(i) => {
            init_options(InitData {
                directory: i.dir,
                your_template: i.template,
            });
        }
        Command::Parse(p) => {
            parse_options(Input { file: p.input }, Output { file: p.output }, p.format)
        }
        Command::Render(_) => todo!(),
        Command::Validate(_) => todo!(),
        Command::Watch(_) => todo!(),
        Command::Lint(_) => todo!(),
    }
}
