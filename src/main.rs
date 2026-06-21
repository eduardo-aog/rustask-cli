//mod services;
mod helpers;
mod enums;
use std::io;
use crate::helpers::helpers::get_date_time;
use enums::task::Task;

fn main() {
    let mut id = 0;
    println!("RusTask Manager");
    println!("Bienvenido");
    loop {
        println!("¿Que quieres hacer? (0: salir, 1: crear tarea)");
        let mut opt: String = String::new();
        io::stdin()
            .read_line(&mut opt)
            .expect("Error al leer la linea");

        let opt: i32 = match opt.trim().parse() {
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
     
            task.show_task();                  
        } else {
            println!("Adios");
            break;
        }

    }

}