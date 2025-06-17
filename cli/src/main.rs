pub mod command;
pub mod flags;

use crate::command::command::{Command, Dsl};
use clap::Parser;

use init::init::InitData;
use lint::lint::LintData;
use parse::parse::ParseFiles;
use render::render::RenderFiles;
use validate::validate::ValidateFile;
use watch::watch::WatchFiles;

fn main() {
    let dsl = Dsl::parse();
    match dsl.command {
        Command::Init(i) => {
            let init_data = InitData {
                directory: i.dir,
                your_template: i.template,
            };
            init_data.init_options();
        }

        Command::Parse(p) => {
            let parse_data = ParseFiles {
                input_file: p.input,
                output_file: p.output,
            };
            parse_data.parse_options(p.format);
        }

        Command::Render(r) => {
            let render_data = RenderFiles {
                input_file: r.input,
                output_file: r.output,
                template_data: r.template,
            };
            render_data.render_options(r.format);
        }

        Command::Validate(v) => {
            let validate_data = ValidateFile { input: v.input };
            validate_data.validate_options();
        }

        Command::Watch(w) => {
            let watch_data = WatchFiles {
                path: w.path,
                on_change: w.on_change,
                output: w.output,
            };
            watch_data.watch_options(w.format);
        }

        Command::Lint(l) => {
            let lint_data = LintData { input: l.input };
            lint_data.lint_options();
        }
    }
}
