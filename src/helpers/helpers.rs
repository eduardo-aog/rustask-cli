use std::io;
use chrono::Local;

pub fn get_date_time() -> String {
    let date = Local::now().naive_local();
    let date: String = date.format("%Y-%m-%d %H:%M:%S").to_string();
    date
}

pub fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut text: String = String::new();
    io::stdin().
        read_line(&mut text)
        .expect("Error al leer la linea");
    return text.trim().to_string()
}

pub fn validate_option() -> i8 {
    loop {
        let opt: String = input("¿Que quieres hacer? \n0: salir \n1: crear tarea \n2: eliminar tarea \n3: mostrar tareas \n4: mostrar tarea con sus metadatos completos \n5: editar nombre \n6: editar estatus\n7: mostrar to-do\n8: mostrar doing\n9: mostrar done\n10: contar todas las tareas\n11: mostrar tarea por ID");
        match opt.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor, ingrese una opción válida");
            }
        }
    }
}