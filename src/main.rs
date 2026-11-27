mod cli;
mod enums;
mod helpers;
mod services;
use clap::Parser;
use cli::commands;
use enums::task_manager::TaskManager;
use services::process;
use std::path::Path;

fn main() {
    let args = commands::Arguments::parse();
    const PATH: &str = "tasks";
    const FILENAME: &str = "tasks.json";
    let complete_path = Path::new(PATH).join(FILENAME);

    let task_manager: TaskManager = process::manager_initialization(PATH, &complete_path);

    process::cli_logic(args, task_manager, &complete_path);
}
