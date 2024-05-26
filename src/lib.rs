mod libs;
use clap::Parser;
use libs::cli::{Cli, Commands};
use libs::functions::{add_task, complete_task, remove_task, show_task};

pub fn run() {
    let cmd = Cli::parse();
    match &cmd.command {
        Commands::Add { name } => {
            match add_task(name.to_string(), false) {
                Ok(_) => println!("Task added"),
                Err(_) => panic!("Fail to add a task"),
            };
        }
        Commands::Ls => show_task(),
        Commands::Ct { id } => complete_task(*id),
        Commands::Rm { id } => remove_task(*id),
    }
}
