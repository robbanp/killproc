use clap::Parser;
use std::process::{Command, Stdio};
use execute::Execute;
use colored::Colorize;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the process to kill
    #[arg(short, long, default_value_t = String::from("vs_code"))]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
#[derive(Debug)]
struct ProcessLine {
    pid: i32,
    name: String,
}

fn main() {

    let _args = Args::parse();
    const COMMAND_PATH: &str = "ps";

    let mut command = Command::new(COMMAND_PATH);
    
    command.arg("--no-headers").arg("-exo").arg("pid,args");
    command.stdout(Stdio::piped());
    let output = String::from_utf8(command.execute_output().unwrap().stdout).unwrap();
    
    // println!("{}", String::from_utf8(output.stdout).unwrap());   
    let mut findings = Vec::new(); 
    for line in output.split("\n") {        

        let row_arr: Vec<&str> = line.trim_start().splitn(2, " ").collect();
        if row_arr.len() < 2 {
            continue;
        }
        let name_str = String::from(row_arr[1]);
        let pid_int = String::from(row_arr[0]).parse::<i32>().unwrap() as i32;

        if name_str.contains(&_args.name) {
            let process: ProcessLine = ProcessLine {name: name_str, pid: pid_int};
//            println!("Row: {:?}", &process);
            findings.push(process);    
        }
        for proc in &findings {
            println!("{} {}, {} {}\n",
             "PID".blue(), 
             proc.pid.to_string().red(),
             "COMMAND".blue(),
             &proc.name[..256].green(),
            )
        }

    }
}
