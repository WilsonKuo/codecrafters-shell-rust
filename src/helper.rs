use crate::path_finder::PathFinder;

use rustyline::{
    Helper, completion::Completer, highlight::Highlighter, hint::Hinter, validate::Validator,
};

pub struct MyHelper;

impl MyHelper {
    pub fn new() -> Self {
        MyHelper
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

        let mut enteries: Vec<String> = Vec::new();
        if matches!(line, "ec" | "ech" | "echo") {
            enteries.push("echo ".to_string());
        } else if matches!(line, "ex" | "exi" | "exit") {
            enteries.push("exit ".to_string());
        } else {
            if let Some(path) = PathFinder::new(line, true).find_executable() {
                if let Some(file_name) = path.file_name() {
                    enteries.push(format!("{} ", file_name.to_string_lossy().into_owned()));
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
