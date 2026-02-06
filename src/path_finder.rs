use is_executable;
use std::path::PathBuf;

pub struct PathFinder {
    candidates: Vec<PathBuf>,
}

impl PathFinder {
    pub fn new(executable: String) -> Self {
        let path_string = std::env::var_os("PATH").expect("PATH environment variable must be set");
        let candidates = std::env::split_paths(&path_string)
            .map(|dir| {
                let mut path = dir.clone();
                path.push(&executable);
                path
            })
            .collect();
        PathFinder { candidates }
    }
    pub fn find_executable(self) -> Option<PathBuf> {
        self.candidates
            .into_iter()
            .find(|candidate| is_executable::is_executable(candidate))
    }
}
