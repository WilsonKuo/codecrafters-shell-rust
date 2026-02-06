mod helper;

use is_executable;
use shlex;
#[allow(unused_imports)]
use std::io::{self, Write};

enum StreamType {
    StandardInput,
    StandardOutput,
    StandardErr,
    StandardOutputAppend,
    StandardErrAppend,
}
struct File {
    f_name: String,
    f_type: StreamType,
}

fn main() {
    let builtin_cmds = ["echo", "exit", "type", "pwd", "cd"];
    let h = helper::MyHelper::new();
    let config = rustyline::Config::builder().build();
    let mut rl = rustyline::Editor::with_config(config).unwrap();
    rl.set_helper(Some(h));

    loop {
        let readline = rl.readline("$ ");
        match readline {
            Ok(line) => {
                let args = shlex::split(&line).unwrap();
                let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                if let Some(cmd) = args.first() {
                    match *cmd {
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
                                                println!(
                                                    "{} is {}",
                                                    cmd_name,
                                                    full_path.to_str().unwrap()
                                                );
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
                        "cd" => {
                            if args.len() > 1 {
                                match args[1] {
                                    "~" => {
                                        if let Some(home_val) = std::env::var_os("HOME") {
                                            std::env::set_current_dir(home_val).unwrap();
                                        }
                                    }
                                    _ => {
                                        if let Err(_) = std::env::set_current_dir(args[1]) {
                                            println!("cd: {}: No such file or directory", args[1])
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            if let Some(path_val) = std::env::var_os("PATH") {
                                let mut find: bool = false;
                                let mut args2 = Vec::new();
                                let mut file = File {
                                    f_name: String::new(),
                                    f_type: StreamType::StandardInput,
                                };

                                let mut v_iter = args[1..].into_iter();
                                while let Some(arg) = v_iter.next() {
                                    match *arg {
                                        ">" | "1>" => {
                                            if let Some(f) = v_iter.next() {
                                                file.f_name = f.to_string();
                                                file.f_type = StreamType::StandardOutput;
                                                break;
                                            }
                                        }
                                        "2>" => {
                                            if let Some(f) = v_iter.next() {
                                                file.f_name = f.to_string();
                                                file.f_type = StreamType::StandardErr;
                                                break;
                                            }
                                        }
                                        ">>" | "1>>" => {
                                            if let Some(f) = v_iter.next() {
                                                file.f_name = f.to_string();
                                                file.f_type = StreamType::StandardOutputAppend;
                                                break;
                                            }
                                        }
                                        "2>>" => {
                                            if let Some(f) = v_iter.next() {
                                                file.f_name = f.to_string();
                                                file.f_type = StreamType::StandardErrAppend;
                                                break;
                                            }
                                        }
                                        _ => args2.push(arg),
                                    }
                                }

                                for path in std::env::split_paths(&path_val) {
                                    let full_path = path.join(*cmd);
                                    if is_executable::is_executable(&full_path) {
                                        find = true;
                                        let output = std::process::Command::new(*cmd)
                                            .args(&args2)
                                            .output()
                                            .expect("failed to execute process");
                                        let stdout = String::from_utf8(output.stdout)
                                            .expect("Not valid UTF-8");
                                        let stderr = String::from_utf8(output.stderr)
                                            .expect("Not valid UTF-8");

                                        match file.f_type {
                                            StreamType::StandardInput => print!("{}", stdout),
                                            StreamType::StandardOutput => {
                                                if let Err(e) =
                                                    std::fs::write(&file.f_name, &stdout)
                                                {
                                                    println!("{:?}", e);
                                                }
                                                print!("{}", stderr);
                                            }
                                            StreamType::StandardErr => {
                                                if let Err(e) =
                                                    std::fs::write(&file.f_name, &stderr)
                                                {
                                                    println!("{:?}", e);
                                                }
                                                print!("{}", stdout);
                                            }
                                            StreamType::StandardOutputAppend => {
                                                if let Ok(mut f) = std::fs::OpenOptions::new()
                                                    .create(true)
                                                    .append(true)
                                                    .open(&file.f_name)
                                                {
                                                    if let Err(_) = f.write(stdout.as_bytes()) {
                                                        // println!("{:?}", e);
                                                    }
                                                }

                                                print!("{}", stderr);
                                            }
                                            StreamType::StandardErrAppend => {
                                                if let Ok(mut f) = std::fs::OpenOptions::new()
                                                    .create(true)
                                                    .append(true)
                                                    .open(&file.f_name)
                                                {
                                                    if let Err(_) = f.write(stderr.as_bytes()) {
                                                        // println!("{:?}", e);
                                                    }
                                                }

                                                print!("{}", stdout);
                                            }
                                        }
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
