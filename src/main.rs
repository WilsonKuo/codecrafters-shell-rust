#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let args: Vec<&str> = input.split_whitespace().collect();
        if let Some(cmd) = args.first() {
            match *cmd {
                "echo" => println!("{}", args[1..].join(" ")),
                "exit" => std::process::exit(0),
                _ => println!("{}: command not found", cmd),
            }
        }
    }
}
