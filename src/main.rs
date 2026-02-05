use is_executable;
use shlex;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let builtin_cmds = ["echo", "exit", "type", "pwd", "cd"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let args = shlex::split(&input).unwrap();
        let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        if let Some(cmd) = args.first() {
            match *cmd {
                "echo" => {
                    if args.len() > 3 && matches!(args[2], ">" | "1>") {
                        if let Err(e) = std::fs::write(&args[3], format!("{}\n", &args[1])) {
                            println!("{:?}", e);
                        };
                    } else {
                        println!("{}", args[1..].join(" "));
                    }
                }
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
                        let mut file = String::new();

                        let mut v_iter = args[1..].into_iter();
                        while let Some(arg) = v_iter.next() {
                            if matches!(arg, &">" | &"1>") {
                                if let Some(f) = v_iter.next() {
                                    file = f.to_string();
                                    break;
                                }
                            } else {
                                args2.push(arg);
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
                                let stdout =
                                    String::from_utf8(output.stdout).expect("Not valid UTF-8");
                                let stderr =
                                    String::from_utf8(output.stderr).expect("Not valid UTF-8");
                                if file.is_empty() {
                                    print!("{}", stdout);
                                } else {
                                    if let Err(e) = std::fs::write(&file, &stdout) {
                                        println!("{:?}", e);
                                    }
                                }
                                print!("{}", stderr);
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
