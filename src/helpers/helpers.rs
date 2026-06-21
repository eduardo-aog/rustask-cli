use chrono::Local;

pub fn get_date_time() -> String {
    let date = Local::now().naive_local();
    let date: String = date.format("%Y-%m-%d %H:%M:%S").to_string();
    date
}