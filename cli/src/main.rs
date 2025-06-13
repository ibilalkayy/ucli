pub mod command;
pub mod flags;

use crate::command::command::{Command, Dsl};
use clap::Parser;

fn main() {
    let dsl = Dsl::parse();
    match dsl.command {
        Command::Init(_) => todo!(),
        Command::Parse(_) => todo!(),
        Command::Render(_) => todo!(),
        Command::Validate(_) => todo!(),
        Command::Watch(_) => todo!(),
        Command::Lint(_) => todo!(),
    }
}
