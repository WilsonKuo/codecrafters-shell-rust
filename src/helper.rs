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
                        entries.push(format!("{}", line));
                        self.counter.set(0);
                    }
                    self.counter.set(current_count + 1);
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
