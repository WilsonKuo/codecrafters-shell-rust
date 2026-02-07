use std::{
    fs::{File, OpenOptions},
    io::{self, Stderr, Stdout},
    path::PathBuf,
};

pub enum Output {
    File(File),
    StdOut(Stdout),
    StdErr(Stderr),
}

pub struct OutputConfig {
    pub stdout: Output,
    pub stderr: Output,
}

impl OutputConfig {
    pub fn new(symbol: &str, file_path: PathBuf) -> Result<Self, ()> {
        match symbol {
            ">" | "1>" => Ok(OutputConfig {
                stdout: Output::File(File::create(file_path).unwrap()),
                stderr: Output::StdErr(io::stderr()),
            }),
            "2>" => Ok(OutputConfig {
                stdout: Output::StdOut(io::stdout()),
                stderr: Output::File(File::create(file_path).unwrap()),
            }),
            ">>" | "1>>" => Ok(OutputConfig {
                stdout: Output::File(
                    OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(file_path)
                        .unwrap(),
                ),
                stderr: Output::StdErr(io::stderr()),
            }),
            "2>>" => Ok(OutputConfig {
                stdout: Output::StdOut(io::stdout()),
                stderr: Output::File(
                    OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(file_path)
                        .unwrap(),
                ),
            }),
            _ => Err(()),
        }
    }
}
