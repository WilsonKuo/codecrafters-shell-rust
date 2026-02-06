mod builtin_command;
mod command;
mod helper;
mod output;
mod path_finder;
mod runner;

#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let h = helper::MyHelper::new();
    let config = rustyline::Config::builder().build();
    let mut rl = rustyline::Editor::with_config(config).unwrap();
    rl.set_helper(Some(h));

    loop {
        let readline = rl.readline("$ ");
        match readline {
            Ok(line) => {
                runner::run(&line);
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
