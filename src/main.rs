use clap::Parser;
use std::process::{Command, Stdio};
use execute::Execute;
use colored::Colorize;
use regex::Regex;
use terminal_menu::{menu, button, run, mut_menu, back_button};


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
    command.arg("--no-headers").arg("aexo").arg("pid,args");
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
            findings.push(process);    
        }
    }
/* 
    for proc in &findings {
        let regex_str = format!(r"({})", _args.name);
        let re = Regex::new(&regex_str).unwrap();

        let new_text = re.replace_all(&proc.name, &_args.name.red().to_string());
        let mut parsed_text = new_text.green().to_string();
        if parsed_text.len() > 256 {
            parsed_text = parsed_text[..256].to_string()
        }

        println!("{} {}, {} {}\n",
         "PID".blue(), 
         proc.pid.to_string().red(),
         "COMMAND".blue(),
         parsed_text,
        )
    }
 */   
    
    let mut menu_collection = vec![];
    menu_collection.push(back_button("Back"));
    for item in &findings {
        menu_collection.push(
            button(format!("{} - {}", item.pid, shorten(&item.name, 64)))
        );
    }
    let menu = menu(menu_collection);

    run(&menu);

    if mut_menu(&menu).selected_item_name() != "Back" {
        let selected_index = mut_menu(&menu).selected_item_index();
        let mut command2 = Command::new("kill"); 
        command2.arg("-9").arg(findings[selected_index - 1].pid.to_string());
        command2.stdout(Stdio::piped());
        let output2 = String::from_utf8(command2.execute_output().unwrap().stdout).unwrap();
        println!("RES: {}", output2);        
    } else {
        println!("Exited")
    }
}

fn shorten(text: &String, len: usize) -> String {
    if text.len() < len {
        return text.to_string();
    }
    text[..len].to_string()
}