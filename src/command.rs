use crate::builtin_command::BuiltinCommand;
use crate::path_finder::PathFinder;

use std::path::PathBuf;

pub enum Command {
    Builtin(BuiltinCommand),
    Executable(PathBuf),
}

impl TryFrom<String> for Command {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Ok(builtin_command) = BuiltinCommand::try_from(value.clone()) {
            Ok(Self::Builtin(builtin_command))
        } else {
            if let Some(path) = PathFinder::new(&value, false).find_executable() {
                Ok(Self::Executable(path))
            } else {
                Err("Not a command".to_string())
            }
        }
    }
}
