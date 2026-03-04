use is_executable;
use std::path::PathBuf;

pub struct PathFinder {
    candidates: Vec<PathBuf>,
}

impl PathFinder {
    pub fn new(executable: &str, prefix: bool) -> Self {
        let path_string = std::env::var_os("PATH").expect("PATH environment variable must be set");
        let mut paths: Vec<PathBuf> = std::env::split_paths(&path_string).collect();
        if let Ok(current_dir) = std::env::current_dir() {
            paths.push(current_dir);
        }
        let candidates: Vec<PathBuf> = if prefix {
            paths
                .into_iter()
                .flat_map(|dir| {
                    std::fs::read_dir(dir)
                        .into_iter()
                        .flatten()
                        .filter_map(|entry| {
                            let entry = entry.ok()?;
                            let file_name = entry.file_name();
                            let file_name_str = file_name.to_str()?;
                            if file_name_str.starts_with(executable) {
                                let path = entry.path();
                                Some(path)
                            } else {
                                None
                            }
                        })
                })
                .collect()
        } else {
            paths
                .into_iter()
                .map(|dir| {
                    let mut path = dir;
                    path.push(executable);
                    path
                })
                .collect()
        };

        PathFinder { candidates }
    }
    pub fn find_file_multiple(self) -> Option<Vec<PathBuf>> {
        if self.candidates.is_empty() {
            None
        } else {
            Some(self.candidates)
        }
    }
    pub fn find_executable(self) -> Option<PathBuf> {
        self.candidates
            .into_iter()
            .find(|candidate| is_executable::is_executable(candidate))
    }
    pub fn find_executable_multiple(self) -> Option<Vec<PathBuf>> {
        let results: Vec<PathBuf> = self
            .candidates
            .into_iter()
            .filter(|candidate| is_executable::is_executable(candidate))
            .collect();
        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }
}
