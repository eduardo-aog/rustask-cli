mod cli;
mod enums;
mod helpers;
mod services;
use crate::helpers::helpers::{ validate_option };
use enums::{ task::Task, task_manager::TaskManager };
use services::data_input;

fn main() {
    println!("RusTask Manager");
    println!("Bienvenido");
    let mut manager: TaskManager = TaskManager::new();
    loop {
        println!("");
        let opt: i8 = validate_option();
        if opt == 0 {
            println!("Adios");
            break;

        } else if opt == 1 {
            data_input::input_add(&mut manager);

        } else if opt == 2 {
            data_input::input_remove(&mut manager);

        } else if opt == 3 {
            manager.show_all_tasks();

        } else if opt == 4 {
            manager.show_all_tasks_with_metadata();

        } else if opt == 5 {
            data_input::input_update_name(&mut manager);

        } else if opt == 6 {
            data_input::input_update_status(&mut manager);

        } else if opt == 7 {
            manager.show_todo();

        } else if opt == 8 {
            manager.show_doing();

        } else if opt == 9 {
            manager.show_done();

        } else if opt == 10 {
            manager.print_summary();

        } else if opt == 11 {
            data_input::input_show_by_id(&mut manager);

        } else { 
            println!("Por favor, ingrese una opción válida");
            continue;
        }
    }

}