pub mod command;
pub mod flags;

use crate::command::command::{Command, Younix};
use clap::Parser;

use cat::cat::CatData;
use grep::grep::GrepData;
use ls::list::ListData;
use sort::sort::SortData;
use tail::tail::TailData;
use view::view::ViewData;
use wc::wc::WcData;

fn handle_commands() {
    let ucli = Younix::parse();
    match ucli.command {
        Command::Cat(i) => {
            let cat_data = CatData {
                path: i.path,
                number: i.number,
            };
            cat_data.cat_options();
        }

        Command::Ls(l) => {
            let list_data = ListData {
                path: l.path,
                all: l.all,
                long: l.long,
            };
            list_data.list_options();
        }

        Command::Grep(g) => {
            let grep_data = GrepData {
                file: g.file,
                pattern: g.pattern,
                case_insensitive: g.case_insensitive,
                invert: g.invert,
                number: g.number,
            };
            grep_data.grep_options();
        }

        Command::View(v) => {
            let view_data = ViewData { file: v.file };
            view_data.view_options();
        }

        Command::Sort(s) => {
            let sort_data = SortData { file: s.file };
            sort_data.sort_options();
        }

        Command::Tail(t) => {
            let tail_data = TailData { path: t.file };
            tail_data.tail_options();
        }

        Command::Wc(w) => {
            let wc_data = WcData { file: w.file };
            wc_data.wc_options();
        }
    }
}

fn main() {
    handle_commands();
}
