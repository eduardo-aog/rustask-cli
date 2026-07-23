mod helpers;
mod enums;
use crate::helpers::helpers::get_date_time;
use crate::helpers::helpers::input;
use enums::task::Task;
use enums::task_manager::TaskManager;

fn main() {
    println!("RusTask Manager");
    println!("Bienvenido");
    let mut manager: TaskManager = TaskManager::new();
    loop {
        // init
        println!("");
        let opt: String = input("¿Que quieres hacer? \n0: salir \n1: crear tarea \n2: eliminar tarea \n3: mostrar tareas \n4: mostrar tarea con sus metadatos completos \n5: editar nombre \n6: editar estatus\n7: mostrar to-do\n8: mostrar doing\n9: mostrar done\n10: contar todas las tareas\n11: mostrar tarea por ID");
        let opt: i8 = match opt.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, ingrese una opción válida");
                continue;
            }
        };

        if opt == 0 {
            println!("Adios");
            break;
        } else if opt == 1 {
            let name: String = input("Indique el nombre de la tarea:");
            let raw_status = input("Indique el estatus de la tarea:");
            let date: String = get_date_time();

            let status: String = match Task::validate_status(raw_status) {
                Ok(valid_status) => valid_status,
                Err(msg) => {
                    println!("{}", msg);
                    continue;
                }
            };

            manager.add_task(name, status, &date, &date);

        } else if opt == 2 {
            let id: String = input("Indique el ID de la tarea que quires eliminar: ");

            let id: i32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Indique un número válido");
                        continue;
                    }
                };
            manager.remove_task(id);

        } else if opt == 3 {
            manager.show_all_tasks();
        } else if opt == 4 {
            manager.show_all_tasks_with_metadata();
        } else if opt == 5 {
            let id: String = input("Edición de nombre: \nIndique el ID de la tarea que quiere editar:"); 

            let id: i32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Indique un número válido");
                        continue;
                    }
                };

            let new_name: String = input(&format!("Indique el nuevo nombre de la tarea {}:", id)); 

            let update: String = get_date_time();
            
            manager.update_task_name(id, new_name.trim().to_string(), update);

        } else if opt == 6 {
            let id: String = input("Edición de estatus: \nIndique el ID de la tarea que quiere editar:");

            let id: i32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Indique un número válido");
                        continue;
                    }
                };

            let raw_status: String = input(&format!("Indique el nuevo estatus de la tarea {}:", id));

            let status: String = match Task::validate_status(raw_status) {
                Ok(valid_status) => valid_status,
                Err(msg) => {
                    println!("{}", msg);
                    continue;
                }
            };
            let update: String = get_date_time();
            
            manager.update_task_status(id, status.trim().to_string(), update);       
    
        } else if opt == 7 {
            manager.show_todo();
        } else if opt == 8 {
            manager.show_doing();
        } else if opt == 9 {
            manager.show_done();
        } else if opt == 10 {
            manager.print_summary();
        } else if opt == 11 {
            let id: String = input("Indique el ID de la tarea a mostrar:");
            let id: i32 = match id.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Por favor, ingrese una opción válida");
                    continue;
                }
            };
            manager.show_by_id(id);
        } else { 
            println!("Por favor, ingrese una opción válida");
            continue;
        }
    }

}