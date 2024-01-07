use execute::Execute;
use std::process::{Command, Stdio};

pub fn shorten(text: &str, len: usize) -> &str {
    if text.len() < len {
        return text;
    }
    &text[..len]
}

pub fn run_command(cmd: &str, args: Vec<&str>) -> anyhow::Result<String> {
    let mut command = Command::new(cmd);
    command.args(args);
    command.stdout(Stdio::piped());
    Ok(
        String::from_utf8(command.execute_output()?.stdout)?
    )
}
