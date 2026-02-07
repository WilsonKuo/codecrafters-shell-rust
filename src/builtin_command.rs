pub enum BuiltinCommand {
    Exit,
    Type,
    Pwd,
    Cd,
    History,
}

impl TryFrom<String> for BuiltinCommand {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "exit" => Ok(BuiltinCommand::Exit),
            "type" => Ok(BuiltinCommand::Type),
            "pwd" => Ok(BuiltinCommand::Pwd),
            "cd" => Ok(BuiltinCommand::Cd),
            "history" => Ok(BuiltinCommand::History),
            _ => Err("Not a builtin command".to_string()),
        }
    }
}
