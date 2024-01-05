mod helpers;
use crate::helpers::*;

use clap::Parser;
use terminal_menu::{menu, button, run, mut_menu, back_button, label};
use crossterm::style::Color;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the process to kill
    #[arg(short, long, default_value_t = String::from("vs_code"))]
    name: String,
}
#[derive(Debug)]
struct ProcessLine {
    pid: i32,
    name: String,
}

fn main() {
    let _args = Args::parse();
    let output = run_command("ps", vec!["--no-headers", "aexo", "pid,args"]);
    let mut findings = Vec::new(); 
    for line in output.split('\n') {        

        let row_arr: Vec<&str> = line.trim_start().splitn(2, ' ').collect();
        if row_arr.len() < 2 {
            continue;
        }
        let name_str = String::from(row_arr[1]);
        let pid_int = String::from(row_arr[0]).parse::<i32>().unwrap();

        if name_str.contains(&_args.name) {
            let process: ProcessLine = ProcessLine {name: name_str, pid: pid_int};
            findings.push(process);    
        }
    }

    let mut menu_collection = vec![];
    menu_collection.push(label("Select process or hit 'q' or esc!").colorize(Color::Red));
    menu_collection.push(back_button("Back"));
    for item in &findings {
        menu_collection.push(
            button(format!("{} - {}", item.pid, shorten(&item.name, 64)))
        );
    }
    let menu = menu(menu_collection);

    run(&menu);
    let cancelled = mut_menu(&menu).canceled();

    if !cancelled && mut_menu(&menu).selected_item_name() != "Back" {
        let selected_index = mut_menu(&menu).selected_item_index();
        let output = run_command("kill", vec!["-9", &findings[selected_index - 1].pid.to_string()]);
        println!("RES: {}", output);        
    } else {
        println!("Exited")
    }
}