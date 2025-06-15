pub mod command;
pub mod flags;

use crate::command::command::{Command, Dsl};
use clap::Parser;

use domains::{
    init::{InitData, init_options},
    parse::{ParseFiles, parse_options},
    render::{RenderFiles, render_options},
    validate::{ValidateFile, validate_options},
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
        Command::Parse(p) => parse_options(
            ParseFiles {
                input_file: p.input,
                output_file: p.output,
            },
            p.format,
        ),
        Command::Render(r) => render_options(
            RenderFiles {
                input_file: r.input,
                output_file: r.output,
                template_data: r.template,
            },
            r.format,
        ),
        Command::Validate(v) => validate_options(ValidateFile { input: v.input }),
        Command::Watch(_) => todo!(),
        Command::Lint(_) => todo!(),
    }
}
