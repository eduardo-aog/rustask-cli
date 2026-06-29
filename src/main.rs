//mod services;
mod helpers;
mod enums;
use std::io;
use crate::helpers::helpers::get_date_time;
use enums::task::Task;
use enums::task_manager::TaskManager;

fn main() {
    let mut id: i32 = 0;
    println!("RusTask Manager");
    println!("Bienvenido");
    let mut manager: TaskManager = TaskManager::new();
    loop {
        // init
        println!("¿Que quieres hacer? \n0: salir \n1: crear tarea \n2: eliminar tarea \n3: mostrar tareas \n4: mostrar tarea con sus metadatos completos \n5: editar nombre \n6: editar estatus");
        let mut opt: String = String::new();
        io::stdin()
            .read_line(&mut opt)
            .expect("Error al leer la linea");

        let opt: i8 = match opt.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, ingrese una opción válida\n");
                continue;
            }
        };
        if opt == 0 {
            println!("Adios");
            break;
        } else if opt == 1 {
            id = id + 1;
            println!("Indique el nombre de la tarea");
            let mut name: String = String::new();

            io::stdin()
                .read_line(&mut name)
                .expect("Error al leer la linea");

            println!("Indique el estatus de la tarea");
            let mut status: String = String::new();

            io::stdin()
                .read_line(&mut status)
                .expect("Error al leer la linea");
            
            let date: String = get_date_time();

            let task: Task = Task::new_task(id, name, status, &date, &date);
            manager.add_task(task);

        } else if opt == 2 {
            println!("Indique el ID de la tarea que quires eliminar: ");
            let mut id: String = String::new();

            io::stdin()
                .read_line(&mut id)
                .expect("Error al leer la linea");

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
            println!("Edición de nombre: \nIndique el ID de la tarea que quiere editar:");
            let mut id: String = String::new();

            io::stdin()
                .read_line(&mut id)
                .expect("Error al leer la linea");

            let id: i32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Indique un número válido");
                        continue;
                    }
                };

            println!("Indique el nuevo nombre de la tarea {}", id);
            let mut new_name: String = String::new();

            io::stdin()
                .read_line(&mut new_name)
                .expect("Error al leer la linea");

            let update: String = get_date_time();
            
            manager.update_task_name(id, new_name, update);

        } if opt == 6 {
            println!("Edición de estatus: \nIndique el ID de la tarea que quiere editar:");
            let mut id: String = String::new();

            io::stdin()
                .read_line(&mut id)
                .expect("Error al leer la linea");

            let id: i32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Indique un número válido");
                        continue;
                    }
                };

            println!("Indique el nuevo estatus de la tarea {}", id);
            let mut new_status: String = String::new();

            io::stdin()
                .read_line(&mut new_status)
                .expect("Error al leer la linea");

            let update: String = get_date_time();
            
            manager.update_task_status(id, new_status, update);       
    
        } 
    }

}