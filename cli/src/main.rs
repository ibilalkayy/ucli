pub mod command;
pub mod flags;

use crate::command::command::{Command, Dsl};
use clap::Parser;

use convert::convert::ConvertData;
use example::example::ExampleData;
use highlight::highlight::HighlightData;
use init::init::InitData;
use parse::parse::ParseData;
use rules::rules::rules_option;
use validate::validate::ValidateData;

fn handle_commands() {
    let dsl = Dsl::parse();
    match dsl.command {
        Command::Init(i) => {
            let init_data = InitData {
                your_path: i.path,
                force: i.force,
            };
            init_data.init_options();
        }

        Command::Validate(v) => {
            let validate_data = ValidateData { file: v.file };
            validate_data.validate_options();
        }

        Command::Parse(p) => {
            let parse_data = ParseData {
                input_file: p.file,
                output_file: p.output,
            };
            parse_data.parse_options();
        }

        Command::Convert(c) => {
            let convert_data = ConvertData {
                file: c.file,
                output: c.output,
            };
            convert_data.convert_options(c.to);
        }

        Command::Highlight(h) => {
            let highlight_data = HighlightData {
                file: h.file,
                theme: h.theme,
            };
            highlight_data.highlight_options();
        }

        Command::Example(e) => {
            let example_data = ExampleData { path: e.path };
            example_data.example_options();
        }

        Command::Rules => {
            rules_option();
        }
    }
}

fn main() {
    handle_commands();
}
