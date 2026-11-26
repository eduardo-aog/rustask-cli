mod cli;
mod enums;
mod helpers;
use clap::Parser;
use cli::commands::{self, Action, EditSubcommand, FilterSubcommand};
use enums::task_manager::TaskManager;
use helpers::helpers::get_date_time;
use std::fs;
use std::path::Path;

fn main() {
    const PATH: &str = "tasks";
    const FILE: &str = "tasks.json";
    let complete_path = Path::new(PATH).join(FILE);

    match fs::create_dir_all(PATH) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error al abrir la carpeta: {}", e);
        }
    };

    let mut task_manager: TaskManager = if !complete_path.exists() {
        let new_manager = TaskManager::new();
        let json = serde_json::to_string_pretty(&new_manager).expect("Error al serializar");
        fs::write(&complete_path, json).expect("Error al escribir archivo");
        new_manager
    } else {
        let content = fs::read_to_string(&complete_path).expect("Error al leer archivo JSON");
        if content.trim().is_empty() {
            let empty_manager = TaskManager::new();
            let json = serde_json::to_string_pretty(&empty_manager).expect("Error al serializar");
            fs::write(&complete_path, json).expect("Error al escribir el archivo");
            empty_manager
        } else {
            serde_json::from_str(&content).expect("Error al parsear el JSON")
        }
    };

    // Falta terminar de leer el archivo y
    // conectarlo con los comandos de CLI

    let args = commands::Arguments::parse();

    match args.entity_type {
        Action::Create(task) => {
            let date: String = get_date_time();
            task_manager.add_task(
                task.task_name,
                task.task_status.cast_to_string(),
                &date,
                &date,
            );
            let updated_content =
                serde_json::to_string_pretty(&task_manager).expect("Error al leer el archivo");
            fs::write(complete_path, updated_content).expect("Error al actualizar tareas");
        }
        Action::Delete(task) => {
            task_manager.remove_task(task.task_id);
        }

        Action::Edit(edit) => match edit.command {
            EditSubcommand::Name(task) => {
                let date: String = get_date_time();
                task_manager.update_task_name(task.task_id, task.new_name, date);
            }

            EditSubcommand::Status(task) => {
                let date: String = get_date_time();
                task_manager.update_task_status(
                    task.task_id,
                    task.new_status.cast_to_string(),
                    date,
                );
            }
        },

        Action::Show(task) => {
            if let Some(id) = task.task_id {
                task_manager.show_by_id(id);
            } else if let Some(filter) = task.filter {
                match filter {
                    FilterSubcommand::Todo => {
                        task_manager.show_todo();
                    }
                    FilterSubcommand::Doing => {
                        task_manager.show_doing();
                    }
                    FilterSubcommand::Done => {
                        task_manager.show_done();
                    }
                    FilterSubcommand::Summary => {
                        task_manager.print_summary();
                    }
                }
            } else if task.complete {
                task_manager.show_all_tasks_with_metadata();
            } else {
                task_manager.show_all_tasks();
            }
        }
    }
}
