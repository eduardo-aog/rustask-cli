use clap:: { 
    Parser,
    Subcommand,
    Args
};

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
    pub task_name: String,

    /// Task status
    pub task_status: String,

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
    pub command: EditSubcommand
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
    pub new_name: String,
}

#[derive(Debug, Args)]
pub struct EditStatus {
    /// Task ID to edit
    pub task_id: i32,
    /// Task new status
    pub new_status: String,
}

/* IMPRESIÓN */
#[derive(Debug, Args)]
pub struct ShowTask {
    /// Show a specific task by ID
    pub task_id: Option<i32>,

    /// Show all tasks with their metadata
    #[clap(long, short = 'c')]
    pub complete: bool,

    /* POR HACER: implementación de filtro (dependiendo del status que diga el usuario)  
    #[clap(subcommand)]
    pub filter: Option<ShowFilter>,
    */
}

/*
#[derive(Debug, Subcommand)]
pub enum ShowFilter {
    /// Show tasks with a specific status
    Status(ShowByStatus),
}

#[derive(Debug, Args)]
pub struct ShowByStatus {
    pub status: String,
}
*/