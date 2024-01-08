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

#[cfg(test)] 
mod test {
    use crate::helpers::shorten;
    use super::run_command;
    
    #[test]
    fn test_shorten() {
        let truth = "hell";
        assert_eq!(shorten("hello", 4),truth)
    }

    #[test]
    fn test_run_command() {
        let output = run_command("ls", vec!["-la"]);
        assert!(output.is_ok())
    }
}
