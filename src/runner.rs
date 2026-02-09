use crate::{
    builtin_command::BuiltinCommand,
    command::Command,
    helper::MyHelper,
    output::{Output, OutputConfig},
};

use rustyline::{Editor, history::FileHistory, history::History};
use std::{
    io::Write,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

const BUILTIN_CMDS: [&str; 6] = ["echo", "exit", "type", "pwd", "cd", "history"];
static HISTORY_IDX: AtomicUsize = AtomicUsize::new(0);

fn exit() {
    std::process::exit(0)
}

fn r#type(args: &Vec<&str>) {
    if let Some(cmd_name) = args.get(1) {
        if BUILTIN_CMDS.contains(cmd_name) {
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

fn pwd() {
    println!("{}", std::env::current_dir().unwrap().display());
}

fn cd(args: &Vec<&str>) {
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

pub fn history(args: &Vec<&str>, rl: &mut Editor<MyHelper, FileHistory>) {
    let history = rl.history();
    match args.get(1).copied() {
        Some("--init") => {
            if let Some(path_string) = std::env::var_os("HISTFILE") {
                if rl.load_history(&path_string).is_err() {
                    print!("fail to read {:?}", path_string);
                    return;
                }
            }
        }
        Some("-r") => {
            // 1. let variable = if let Pattern = Scrutinee { ValueIfMatch } else { ValueIfNoMatch };
            // 2. let variable = if Condition { ValueIfTrue } else { ValueIfFalse };
            // 3. Rust 1.65 (https://blog.rust-lang.org/2022/11/03/Rust-1.65.0/): let-else statements
            let Some(file_name) = args.get(2) else {
                print!("please provide history file name");
                return;
            };

            if let Err(_) = rl.load_history(file_name) {
                print!("fail to read {}", file_name);
                return;
            };
        }
        Some("-w") => {
            let Some(file_name) = args.get(2) else {
                print!("please provide history file name");
                return;
            };

            // Saves "#V2" as a flag in the first line.
            // Since this flag causes test failures, we must manually write the file content instead.
            // if let Err(_) = rl.save_history(file_name) {
            //     print!("fail to write {}", file_name);
            //     return;
            // }
            let mut file = std::fs::File::create(file_name).unwrap();
            for line in history {
                std::writeln!(file, "{}", line).unwrap();
            }
        }
        Some("-a") => {
            let Some(file_name) = args.get(2) else {
                print!("please provide history file name");
                return;
            };

            // Saves "#V2" as a flag in the first line.
            // Since this flag causes test failures, we must manually write the file content instead.
            // if let Err(_) = rl.append_history(std::path::Path::new(file_name)) {
            //     print!("fail to append {}", file_name);
            //     return;
            // }

            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_name)
                .unwrap();
            for entry in rl
                .history()
                .iter()
                .skip(HISTORY_IDX.load(Ordering::Relaxed))
            {
                writeln!(file, "{}", entry).unwrap();
            }

            HISTORY_IDX.store(history.len(), Ordering::Relaxed);
        }
        _ => {
            // Handle the following cases:
            // $ history
            // $ history <n>
            let total = history.len();
            let start_index = args
                .get(1)
                .and_then(|s| s.parse::<usize>().ok())
                .map(|n| total.saturating_sub(n))
                .unwrap_or(0);

            for (index, entry) in rl.history().into_iter().enumerate().skip(start_index) {
                println!("\t{}  {}", index + 1, entry);
            }
        }
    }
}

pub fn cmd(args: &Vec<&str>, command_path: PathBuf) {
    let mut v_iter = args[1..].into_iter();
    let mut args2 = Vec::new();
    let mut symbol = "";
    let mut file_path = PathBuf::new();
    while let Some(&arg) = v_iter.next() {
        if arg.contains(">") {
            if let Some(f) = v_iter.next() {
                file_path.push(f);
                symbol = arg;
            }
        } else {
            args2.push(arg);
        }
    }

    let output = std::process::Command::new(
        command_path
            .file_name()
            .unwrap() // This is already validated during the construction of the Command object
            .to_string_lossy()
            .into_owned(),
    )
    .args(&args2)
    .output()
    .expect("failed to execute process");
    let stdout_string = String::from_utf8(output.stdout).expect("Not valid UTF-8");
    let stderr_string = String::from_utf8(output.stderr).expect("Not valid UTF-8");
    if let Ok(output_config) = OutputConfig::new(symbol, file_path) {
        match output_config.stdout {
            Output::File(mut file) => {
                file.write(stdout_string.as_bytes()).unwrap();
            }
            Output::StdOut(mut stdout) => {
                stdout.write(stdout_string.as_bytes()).unwrap();
            }
            Output::StdErr(_) => {
                panic!();
            }
        }
        match output_config.stderr {
            Output::File(mut file) => {
                file.write(stderr_string.as_bytes()).unwrap();
            }
            Output::StdOut(_) => {
                panic!();
            }
            Output::StdErr(mut stderr) => {
                stderr.write(stderr_string.as_bytes()).unwrap();
            }
        }
    } else {
        print!("{}", stdout_string);
        print!("{}", stderr_string);
    }
}

pub fn run(line: &String, rl: &mut Editor<MyHelper, FileHistory>) {
    if let Ok(_) = rl.add_history_entry(line.clone()) {}
    let args = shlex::split(line).unwrap();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    if let Some(command_string) = args.first() {
        if let Ok(command) = Command::try_from(command_string.to_string()) {
            match command {
                Command::Builtin(BuiltinCommand::Exit) => exit(),
                Command::Builtin(BuiltinCommand::Type) => r#type(&args),
                Command::Builtin(BuiltinCommand::Pwd) => pwd(),
                Command::Builtin(BuiltinCommand::Cd) => cd(&args),
                Command::Builtin(BuiltinCommand::History) => history(&args, rl),
                Command::Executable(command_path) => cmd(&args, command_path),
            }
        } else {
            println!("{}: command not found", command_string);
        }
    }
}
