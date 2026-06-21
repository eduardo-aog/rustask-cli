pub struct Task {
    id: i32,
    name: String,
    status: String,
    created_at: String,
    updated_at: String,
}

impl Task {
    pub fn new_task(id: i32, name: String, status: String, created_at: &str, updated_at: &str,) -> Task {
        Task {
            id,
            name,
            status,
            created_at: created_at.to_string(),
            updated_at: updated_at.to_string(),
        }
    }

    pub fn show_task(&self) {
        println!("[{}] - {} | {} - (creado: {}, modificado {})", self.id, self.name, self.status, self.created_at, self.updated_at)
    }
}