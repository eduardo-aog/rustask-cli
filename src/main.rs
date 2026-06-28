//mod services;
mod helpers;
mod enums;
use std::io;
use crate::helpers::helpers::get_date_time;
use enums::task::Task;
use enums::task_manager::TaskManager;

fn main() {
    let mut id = 0;
    println!("RusTask Manager");
    println!("Bienvenido");
    let mut manager: TaskManager = TaskManager::new();
    loop {
        // init
        println!("¿Que quieres hacer? (0: salir, 1: crear tarea, 2: eliminar tarea, 3: mostrar tareas, 4: modificación de tareas)");
        let mut opt: String = String::new();
        io::stdin()
            .read_line(&mut opt)
            .expect("Error al leer la linea");

        let opt: i8 = match opt.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: por favor ingrese un numero valido");
                continue;
            }
        };
        
        if opt == 1 {
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
            
        } else {
            println!("Adios");
            break;
        }

    }

}