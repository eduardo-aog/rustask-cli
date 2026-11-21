mod cli;
mod enums;
mod helpers;
use crate::cli::commands::EditSubcommand;
use clap::Parser;
use cli::commands;
use cli::commands::{Action, FilterSubcommand};
use enums::task_manager::TaskManager;
use helpers::helpers::get_date_time;

fn main() {
    let mut manager: TaskManager = TaskManager::new();

    let args = commands::Arguments::parse();

    match args.entity_type {
        Action::Create(task) => {
            let date: String = get_date_time();
            manager.add_task(
                task.task_name,
                task.task_status.cast_to_string(),
                &date,
                &date,
            );
        }
        Action::Delete(task) => {
            manager.remove_task(task.task_id);
        }

        Action::Edit(edit) => match edit.command {
            EditSubcommand::Name(task) => {
                let date: String = get_date_time();
                manager.update_task_name(task.task_id, task.new_name, date);
            }

            EditSubcommand::Status(task) => {
                let date: String = get_date_time();
                manager.update_task_status(task.task_id, task.new_status.cast_to_string(), date);
            }
        },

        Action::Show(task) => {
            if let Some(id) = task.task_id {
                manager.show_by_id(id);
            } else if let Some(filter) = task.filter {
                match filter {
                    FilterSubcommand::Todo => {
                        manager.show_todo();
                    }
                    FilterSubcommand::Doing => {
                        manager.show_doing();
                    }
                    FilterSubcommand::Done => {
                        manager.show_done();
                    }
                    FilterSubcommand::Summary => {
                        manager.print_summary();
                    }
                }
            } else if task.complete {
                manager.show_all_tasks_with_metadata();
            } else {
                manager.show_all_tasks();
            }
        }
    }
}
