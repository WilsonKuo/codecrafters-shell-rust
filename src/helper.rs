use std::cell::{Cell, RefCell};

use crate::path_finder::PathFinder;

use rustyline::{
    Helper, completion::Completer, highlight::Highlighter, hint::Hinter, validate::Validator,
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

        let mut enteries: Vec<String> = Vec::new();
        if matches!(line, "ec" | "ech" | "echo") {
            enteries.push("echo ".to_string());
        } else if matches!(line, "ex" | "exi" | "exit") {
            enteries.push("exit ".to_string());
        } else {
            if let Some(paths) = PathFinder::new(line, true).find_executable_multiple() {
                let mut file_names: Vec<String> = Vec::new();
                paths.iter().for_each(|path| {
                    if let Some(file_name) = path.file_name() {
                        file_names.push(file_name.to_string_lossy().into_owned());
                    }
                });
                file_names.sort();

                let same_prefix = file_names.windows(2).all(|w| w[1].starts_with(&w[0]));
                if same_prefix {
                    file_names.iter().for_each(|file_name| {
                        if file_name.len() > line.len() {
                            enteries.push(file_name.clone());
                        }
                    });
                    if let Some(last_entry) = enteries.last_mut() {
                        last_entry.push(' ');
                    }
                    return Ok((0, enteries));
                }

                if file_names.len() == 1 {
                    enteries.push(format!("{} ", file_names[0]));
                } else {
                    let current_count = self.counter.get();
                    if current_count == 0 {
                        print!("\x07");
                    } else {
                        println!("\n{}", file_names.join("  "));
                        enteries.push(format!("{}", line));
                        self.counter.set(0);
                    }
                    self.counter.set(current_count + 1);
                }
            } else {
                print!("\x07");
            }
        }

        Ok((0, enteries))
    }
}
impl Highlighter for MyHelper {}
impl Hinter for MyHelper {
    type Hint = String;
}
impl Validator for MyHelper {}
