use crate::helpers::helpers::{ get_date_time, input };
use crate::enums::{ task::Task, task_manager::TaskManager };

pub fn input_add(manager: &mut TaskManager) {
    let name: String = input("Indique el nombre de la tarea:");
    let raw_status = input("Indique el estatus de la tarea:");
    let date: String = get_date_time();

    let status: String = match Task::validate_status(raw_status) {
        Ok(valid_status) => valid_status,
        Err(msg) => {
            println!("{}", msg);
            return
        }
    };
    manager.add_task(name, status, &date, &date);
}

pub fn input_remove(manager: &mut TaskManager) {
    let id: String = input("Indique el ID de la tarea que quires eliminar: ");
    let id: i32 = match id.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Indique un número válido");
            return
        }
    };
    manager.remove_task(id);
}

pub fn input_update_name(manager: &mut TaskManager) {
    let id: String = input("Edición de nombre: \nIndique el ID de la tarea que quiere editar:"); 
    let id: i32 = match id.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Indique un número válido");
                return
            }
        };
    let new_name: String = input(&format!("Indique el nuevo nombre de la tarea {}:", id)); 
    let update: String = get_date_time();
    manager.update_task_name(id, new_name.trim().to_string(), update);
}

pub fn input_update_status(manager: &mut TaskManager) {
    let id: String = input("Edición de estatus: \nIndique el ID de la tarea que quiere editar:");
    let id: i32 = match id.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Indique un número válido");
                return
            }
        };
    let raw_status: String = input(&format!("Indique el nuevo estatus de la tarea {}:", id));
    let status: String = match Task::validate_status(raw_status) {
        Ok(valid_status) => valid_status,
        Err(msg) => {
            println!("{}", msg);
            return
        }
    };
    let update: String = get_date_time();
    manager.update_task_status(id, status.trim().to_string(), update);   
}

pub fn input_show_by_id(manager: &mut TaskManager) {
    let id: String = input("Indique el ID de la tarea a mostrar:");
    let id: i32 = match id.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Por favor, ingrese una opción válida");
            return
        }
    };
    manager.show_by_id(id);
}