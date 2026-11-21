use crate::enums::task::Task;
use clap::ValueEnum;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Clone, ValueEnum)]
pub enum TaskStatus {
    Todo,
    Doing,
    Done,
}

impl TaskStatus {
    pub fn cast_to_string(&self) -> String {
        match self {
            TaskStatus::Todo => "todo".to_string(),
            TaskStatus::Doing => "doing".to_string(),
            TaskStatus::Done => "done".to_string(),
        }
    }
}

/// Initialization of
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[clap(subcommand)]
    pub entity_type: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Create a new task
    Create(CreateTask),
    /// Delete a task by its ID
    Delete(DeleteTask),
    /// Edit tasks names and statuses
    Edit(EditTask),
    /// Show tasks
    Show(ShowTask),
}

/* CREAR */
#[derive(Debug, Args)]
pub struct CreateTask {
    /// Task title
    #[arg(value_parser = Task::validate_non_empty)]
    pub task_name: String,

    /// Task status
    pub task_status: TaskStatus,
}

/* ELIMINAR */
#[derive(Debug, Args)]
pub struct DeleteTask {
    /// Task to delete (by ID)
    pub task_id: i32,
}

/* EDICIÓN */
#[derive(Debug, Args)]
pub struct EditTask {
    #[clap(subcommand)]
    pub command: EditSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum EditSubcommand {
    /// Edit task's name
    Name(EditName),
    /// Edit taks's status
    Status(EditStatus),
}

#[derive(Debug, Args)]
pub struct EditName {
    /// Task ID to edit
    pub task_id: i32,
    /// Task new name
    #[arg(value_parser = Task::validate_non_empty)]
    pub new_name: String,
}

#[derive(Debug, Args)]
pub struct EditStatus {
    /// Task ID to edit
    pub task_id: i32,
    /// Task new status
    pub new_status: TaskStatus,
}

/* IMPRESIÓN */
#[derive(Debug, Args)]
pub struct ShowTask {
    /// Show a specific task by ID
    pub task_id: Option<i32>,

    /// Show all tasks with their metadata
    #[clap(long, short = 'c')]
    pub complete: bool,

    #[clap(subcommand)]
    pub filter: Option<FilterSubcommand>,
}

#[derive(Debug, Subcommand)]
pub enum FilterSubcommand {
    /// Show "To-do" tasks
    Todo,
    /// Show "Doing" tasks
    Doing,
    /// Show "Done" tasks
    Done,
    /// Show summary of tasks
    Summary,
}
