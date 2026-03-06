use std::path::PathBuf;

pub struct PathCompletor {
    candidates: Vec<PathBuf>,
}

impl PathCompletor {
    pub fn new(target: &str) -> Self {
        let Ok(mut current_dir) = std::env::current_dir() else {
            panic!("current_dir is not working");
        };
        let current_dir_ori = current_dir.clone();

        let target_split: Vec<&str> = target.split('/').collect();
        let post;
        if let Some((last, head)) = target_split.split_last() {
            for h in head {
                current_dir.push(h);
            }
            post = last;
        } else {
            post = &target;
        }
        let candidates: Vec<PathBuf> = std::fs::read_dir(current_dir)
            .into_iter()
            .flatten()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_name = entry.file_name();
                let file_name_str = file_name.to_str()?;

                if file_name_str.starts_with(post) {
                    let path = entry.path();
                    let stripped_path = path.strip_prefix(&current_dir_ori).ok()?.to_path_buf();
                    Some(stripped_path)
                } else {
                    None
                }
            })
            .collect();
        Self { candidates }
    }
    pub fn find_path_multiple(self) -> Option<Vec<PathBuf>> {
        if self.candidates.is_empty() {
            None
        } else {
            Some(self.candidates)
        }
    }
}
