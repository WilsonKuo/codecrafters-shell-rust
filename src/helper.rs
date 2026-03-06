use std::cell::{Cell, RefCell};

use crate::arg_completor::ArgCompletor;
use crate::path_finder::PathFinder;

use rustyline::{
    completion::Completer, highlight::Highlighter, hint::Hinter, validate::Validator, Helper,
};

pub struct MyHelper {
    last_line: RefCell<String>,
    counter: Cell<i32>,
}

impl MyHelper {
    pub fn new() -> Self {
        MyHelper {
            last_line: RefCell::new(String::new()),
            counter: Cell::new(0),
        }
    }
}

impl Helper for MyHelper {}
impl Completer for MyHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let _ = (line, pos, ctx);

        if *self.last_line.borrow() != line {
            self.counter.set(0);
            *self.last_line.borrow_mut() = line.to_string();
        }

        let mut entries: Vec<String> = Vec::new();
        if matches!(line, "ec" | "ech" | "echo") {
            entries.push("echo ".to_string());
        } else if matches!(line, "ex" | "exi" | "exit") {
            entries.push("exit ".to_string());
        } else {
            if let Some(paths) = PathFinder::new(line, true).find_executable_multiple() {
                let mut file_names: Vec<String> = Vec::new();
                file_names.extend(paths.iter().filter_map(|path| {
                    path.file_name()
                        .map(|file_name_string| file_name_string.to_string_lossy().into_owned())
                }));
                file_names.sort();

                // Partial completions
                let same_prefix = file_names.windows(2).all(|w| w[1].starts_with(&w[0]));
                if same_prefix {
                    entries.extend(
                        file_names
                            .iter()
                            .filter(|file_name| file_name.len() > line.len())
                            .cloned(),
                    );

                    if let Some(last_entry) = entries.last_mut() {
                        last_entry.push(' ');
                    }
                    return Ok((0, entries));
                }

                // Executable completion
                if file_names.len() == 1 {
                    entries.push(format!("{} ", file_names[0]));
                } else {
                    // Multiple completions
                    let current_count = self.counter.get();
                    if current_count > 0 {
                        println!("\n{}", file_names.join("  "));
                        entries.push(line.to_string());
                        self.counter.set(0);
                    }
                    self.counter.set(current_count + 1);
                }
            }
        }

        let substrs: Vec<&str> = line.split(" ").filter(|&c| !c.is_empty()).collect();
        if !substrs.is_empty()
            && PathFinder::new(substrs[0], false)
                .find_executable()
                .is_none()
        {
            return Ok((0, entries));
        }
        if substrs.len() == 1 {
            if let Some(paths) = ArgCompletor::new("").find_arg_multiple() {
                for path in paths {
                    let postfix = if path.is_dir() { "/" } else { " " };
                    let mut space = "";
                    if !line.ends_with(" ") {
                        space = " ";
                    }
                    let replaced_line =
                        format!("{}{}{}{}", line, space, path.to_str().unwrap(), postfix);
                    entries.push(replaced_line);
                }
            }
        } else if substrs.len() > 1 {
            let curr_arg = substrs.last().unwrap();
            if let Some(mut paths) = ArgCompletor::new(curr_arg).find_arg_multiple() {
                paths.sort();
                // File completion
                if paths.len() == 1 {
                    let postfix = if paths[0].is_dir() { "/" } else { " " };
                    let replaced_line = format!(
                        "{}{}",
                        line.replace(curr_arg, paths[0].to_str().unwrap()),
                        postfix
                    );
                    entries.push(replaced_line);
                } else {
                    // Partial completions
                    let same_prefix = paths
                        .windows(2)
                        .all(|w| w[1].to_str().unwrap().starts_with(w[0].to_str().unwrap()));
                    if same_prefix {
                        let mut idx = 0;
                        for (curr_idx, path) in paths.iter().enumerate() {
                            if path.to_str().unwrap().len() > curr_arg.len() {
                                idx = curr_idx;
                                break;
                            }
                        }
                        let postfix = if paths[idx].is_dir() { "" } else { " " };
                        let replaced_line = format!(
                            "{}{}",
                            line.replace(curr_arg, paths[idx].to_str().unwrap()),
                            postfix
                        );
                        entries.push(replaced_line);
                        return Ok((0, entries));
                    }
                    // Multiple matches
                    for path in paths {
                        let postfix = if path.is_dir() { "/" } else { "" };
                        let replaced_line = format!(
                            "{}{}",
                            line.replace(curr_arg, path.to_str().unwrap())
                                .replace(substrs[0], "")
                                .trim(),
                            postfix
                        );
                        entries.push(replaced_line);
                    }
                }
            }
        }

        Ok((0, entries))
    }
}
impl Highlighter for MyHelper {}
impl Hinter for MyHelper {
    type Hint = String;
}
impl Validator for MyHelper {}
