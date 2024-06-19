#![allow(dead_code)]

use colored::*;
use dialoguer::{Confirm, console::Term};
use log::LevelFilter;
use simple_logger::SimpleLogger;

use process_manager::{ProcessAction, ProcessManager};

mod process_manager;
mod directory_cleaner;
mod reset_adsk;

fn main() {
    println!("{}", "****************************************".blue().bold());
    println!("{}", "*      Anydesk Waiting Time Bypass     *".blue().bold());
    println!("{}", "*            by Asman Mirza            *".blue().bold());
    println!("{}", "****************************************".blue().bold());

    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let term = Term::stdout();
    if Confirm::new()
        .with_prompt("This will kill the Anydesk process. Are you willing to continue?")
        .default(false)
        .interact_on(&term)
        .unwrap()
    {
        ProcessManager::manage_anydesk(ProcessAction::Kill);


        if Confirm::new()
            .with_prompt("Do you want to start the Anydesk process?")
            .default(false)
            .interact_on(&term)
            .unwrap()
        {
            ProcessManager::manage_anydesk(ProcessAction::Start);

            println!("{}", "Anydesk process started.".green().bold());
        }

        println!("{}", "Operation was successful.".green().bold());
    } else {
        println!("{}", "Operation cancelled by user.".red().bold());
    }
}
