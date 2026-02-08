mod builtin_command;
mod command;
mod helper;
mod output;
mod path_finder;
mod runner;

use crate::helper::MyHelper;

#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let h = helper::MyHelper::new();
    let config = rustyline::Config::builder().build();
    let m_history = rustyline::history::MemHistory::new();
    let mut rl: rustyline::Editor<MyHelper, rustyline::history::MemHistory> =
        rustyline::Editor::with_history(config, m_history).unwrap();
    rl.set_helper(Some(h));

    loop {
        let readline = rl.readline("$ ");
        match readline {
            Ok(line) => {
                runner::run(&line, &mut rl);
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("Interrupted");
                std::process::exit(0);
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("Encountered Eof");
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
    }
}
