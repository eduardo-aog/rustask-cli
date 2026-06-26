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

    pub fn add_task(&mut self, task: Task) {
        self.task_manager.push(task);
        self.size = self.size+1
    }

    // Eliminación de tareas
    pub fn remove_task(&mut self, task_id: i32) {
        if let Some(pos) = self.task_manager.iter().position(|t| t.id == task_id) {
            self.task_manager.remove(pos);
            self.size -= 1;
            println!("Tarea removida exitosamente")
        } else {
            println!("No se encontró una tarea con id {}", task_id);
        }
    }
}