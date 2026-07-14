use std::io::{self, Read};
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