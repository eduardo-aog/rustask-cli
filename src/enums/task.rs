use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: i32,
    name: String,
    status: String,
    created_at: String,
    updated_at: String,
}

impl Task {
    // Creación de tareas
    pub fn new_task(
        id: i32,
        name: String,
        status: String,
        created_at: &str,
        updated_at: &str,
    ) -> Task {
        Task {
            id,
            name,
            status,
            created_at: created_at.to_string(),
            updated_at: updated_at.to_string(),
        }
    }

    // Actualización y cambios en tareas
    pub fn change_task_name(&mut self, new_name: String, update: String) {
        self.name = new_name;
        self.updated_at = update;
    }

    pub fn change_task_status(&mut self, new_status: String, update: String) {
        self.status = new_status;
        self.updated_at = update;
    }

    // Mostrar tareas
    pub fn show_task(&self) {
        println!("[{}] - {} \n | {}", self.id, self.name, self.status)
    }

    pub fn show_task_with_metadata(&self) {
        println!(
            "[{}] - {} \n | {} \n(creado: {}, modificado {})",
            self.id, self.name, self.status, self.created_at, self.updated_at
        )
    }

    // Getters
    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_status(&self) -> String {
        self.status.to_string()
    }

    // Validaciones propias
    pub fn validate_non_empty(val: &str) -> Result<String, String> {
        let trimmed = val.trim();
        if trimmed.is_empty() {
            Err("El nombre no puede estar vacío".to_string())
        } else {
            // Retorna el texto limpio sin espacios sobrantes en los extremos
            Ok(trimmed.to_string())
        }
    }
}
