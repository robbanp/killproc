mod helpers;
use crate::helpers::*;

use clap::Parser;
use crossterm::style::Color;
use terminal_menu::{back_button, button, label, menu, mut_menu, run};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // change so -n is not needed
    /// Name of the process to kill
    #[arg(short, long, default_value_t = String::from("vs_code"))]
    name: String,
}
#[derive(Debug)]
struct ProcessLine {
    pid: i32,
    name: String,
}

fn main() -> anyhow::Result<()> {
    let _args = Args::parse();
    let output = run_command("ps", vec!["--no-headers", "aexo", "pid,args"])?;
    let mut findings = Vec::new();
    for line in output.lines() {
        let Some((pid, name)) = line.trim_start().split_once(' ') else {
            continue;
        };
        let name_str = String::from(name);
        let pid_int = pid.parse::<i32>()?;

        if name_str.contains(&_args.name) {
            let process: ProcessLine = ProcessLine {
                name: name_str,
                pid: pid_int,
            };
            findings.push(process);
        }
    }

    let mut menu_collection = vec![
        label("Select process or hit 'q' or esc!").colorize(Color::Red),
        back_button("Back"),  
    ];
    for item in &findings {
        menu_collection.push(button(format!(
            "{} - {}",
            item.pid,
            shorten(&item.name, 64)
        )));
    }
    let menu = menu(menu_collection);

    run(&menu);
    let menu = mut_menu(&menu);
    let index = menu.selected_item_index() - 1;

    if index == 0 || menu.canceled() {
        println!("Exited");
        return Ok(());
    }

    if let Some(item) = findings.get(index) {
        let output = run_command("kill", vec!["-9", &item.pid.to_string()])?;
        println!("RES: {output}");
    } else {
        println!("Could not find {}", menu.selected_item_name());
    }

    return Ok(());
}
