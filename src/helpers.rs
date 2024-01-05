pub mod helpers {
    use std::process::{Command, Stdio};
    use execute::Execute;
    
    pub fn shorten(text: &String, len: usize) -> String {
        if text.len() < len {
            return text.to_string();
        }
        text[..len].to_string()
    }
    
    pub fn run_command(cmd: &str, args: Vec<&str>) -> String {
        let mut command = Command::new(cmd); 
        command.args(args);
        command.stdout(Stdio::piped());
        String::from_utf8(command.execute_output().unwrap().stdout).unwrap()
    }
}