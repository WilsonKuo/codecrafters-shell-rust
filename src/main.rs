use is_executable;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let builtin_cmds = ["echo", "exit", "type", "pwd"];

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
                "type" => {
                    if let Some(cmd_name) = args.get(1) {
                        if builtin_cmds.contains(cmd_name) {
                            println!("{} is a shell builtin", cmd_name);
                        } else {
                            if let Some(path_val) = std::env::var_os("PATH") {
                                let mut find: bool = false;
                                for path in std::env::split_paths(&path_val) {
                                    let full_path = path.join(cmd_name);
                                    if is_executable::is_executable(&full_path) {
                                        println!("{} is {}", cmd_name, full_path.to_str().unwrap());
                                        find = true;
                                        break;
                                    }
                                }
                                if !find {
                                    println!("{}: not found", cmd_name)
                                }
                            }
                        }
                    }
                }
                "pwd" => {
                    println!("{}", std::env::current_dir().unwrap().display());
                }
                _ => {
                    if let Some(path_val) = std::env::var_os("PATH") {
                        let mut find: bool = false;
                        for path in std::env::split_paths(&path_val) {
                            let full_path = path.join(*cmd);
                            if is_executable::is_executable(&full_path) {
                                find = true;
                                let output = std::process::Command::new(*cmd)
                                    .args(&args[1..])
                                    .output()
                                    .expect("failed to execute process")
                                    .stdout;
                                print!("{}", String::from_utf8(output).expect("Not valid UTF-8"));
                                break;
                            }
                        }
                        if !find {
                            println!("{}: command not found", cmd);
                        }
                    }
                }
            }
        }
    }
}
