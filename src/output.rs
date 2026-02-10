use std::{
    fs::File,
    io::{self, Stderr, Stdout, Write},
};

pub enum Output {
    File(File),
    StdOut(Stdout),
    StdErr(Stderr),
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Output::StdOut(out) => out.write(buf),
            Output::StdErr(err) => err.write(buf),
            Output::File(f) => f.write(buf),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        match self {
            Output::StdOut(out) => out.flush(),
            Output::StdErr(err) => err.flush(),
            Output::File(f) => f.flush(),
        }
    }
}
