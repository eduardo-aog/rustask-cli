use crate::Task;
pub struct TaskManager {
    task_manager: Vec<Task>,
    size: i32
}

impl TaskManager {

    pub fn new() -> TaskManager {
        TaskManager {
            task_manager: Vec::new(),
            size: 0
        }
    }

    // Añadir tareas
    pub fn add_task(&mut self, task: Task) {
        self.task_manager.push(task);
        self.size = self.size+1
    }

    // Mostrar tareas
    pub fn show_all_tasks(&self) {
        if self.task_manager.len() == 0 {
            println!("El administrador está vacío, por favor agregue una tarea");
            return
        }
        for element in &self.task_manager {
            element.show_task();
        }
    }

    // Eliminación de tareas
    pub fn remove_task(&mut self, task_id: i32) {
        if let Some(pos) = self.task_manager.iter().position(|t| t.id == task_id) {
            self.task_manager.remove(pos);
            self.size -= 1;
            println!("Tarea removida exitosamente")
        } else {
            println!("No se encontró una tarea con id {}", task_id)
        }
    }
}