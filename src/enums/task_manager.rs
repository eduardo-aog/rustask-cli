use crate::Task;
const TASK_NULL: usize = usize::MAX;
pub struct TaskManager {
    task_manager: Vec<Task>,
    size: i32,
    next_id: i32,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            task_manager: Vec::new(),
            size: 0,
            next_id: 1
        }
    }

    // Genera el ID para colocarselo a la tarea y sumar al siguiente
    pub fn generate_id(&mut self) -> i32 {
        let id: i32 = self.next_id;
        self.next_id += 1;
        id
    }

    // Añadir tareas
    pub fn add_task(&mut self, name: String, status: String, created_at: &str, updated_at: &str,) {
        let task: Task = Task::new_task(self.generate_id(), name, status, created_at, updated_at);
        self.task_manager.push(task);
        self.size = self.size+1;
        println!("Tarea añadida exitosamente")
    }

    // Mostrar tareas
    pub fn show_all_tasks(&self) {
        if self.size == 0 {
            println!("El administrador está vacío, por favor, agregue una tarea");
            return
        }
        for element in &self.task_manager {
            element.show_task();
        }
    }

    pub fn show_todo(&self) {
        if self.size == 0 {
            println!("El administrador está vacío, por favor, agregue una tarea");
            return
        }
        for element in &self.task_manager {
            if element.get_status() == "todo" {
                element.show_task();
            }
        }
    }

    pub fn show_doing(&self) {
        if self.size == 0 {
            println!("El administrador está vacío, por favor, agregue una tarea");
            return
        }
        for element in &self.task_manager {
            if element.get_status() == "doing" {
                element.show_task();
            }
        }
    }

    pub fn show_done(&self) {
        if self.size == 0 {
            println!("El administrador está vacío, por favor, agregue una tarea");
            return
        }
        for element in &self.task_manager {
            if element.get_status() == "done" {
                element.show_task();
            }
        }
    }

    pub fn count_by_status(&self, status: &str) -> usize {
        self.task_manager
            .iter()
            .filter(|task| task.get_status() == status)
            .count()
    }

    pub fn print_summary(&self) {
        println!("{} tasks", self.size);
        println!("{} todo, {} doing, {} done",
                self.count_by_status("todo"),
                self.count_by_status("doing"),
                self.count_by_status("done"),
        )
    }

    pub fn show_all_tasks_with_metadata(&self) {
        if self.size == 0 {
            println!("El administrador está vació, por favor, agregue una tarea");
            return
        }
        for element in &self.task_manager {
            element.show_task_with_metadata();
        }
    }

    // Eliminación de tareas
    pub fn remove_task(&mut self, task_id: i32) {
        let position: usize = self.get_index_on_vector(task_id);
        if position == TASK_NULL {
            return
        }
        self.task_manager.remove(position);
        self.size -= 1;
        println!("Tarea con ID {} removida exitosamente", task_id);
    }

    // Actualizar tareas (nombre/estatus)
    pub fn update_task_name(&mut self, task_id: i32, new_name: String, update: String) {
        let position: usize = self.get_index_on_vector(task_id);
        if position == TASK_NULL {
            return
        }
        if let Some(task) = self.task_manager.get_mut(position) {
            task.change_task_name(new_name, update);
            println!("Nombre actualizado exitosamente")
        }
    }

    pub fn update_task_status(&mut self, task_id: i32, new_status: String, update: String) {
        let position: usize = self.get_index_on_vector(task_id);
        if position == TASK_NULL {
            return
        }
        if let Some(task) = self.task_manager.get_mut(position) {
            task.change_task_status(new_status, update);
            println!("Estatus actualizado exitosamente")
        }
    }

    // Utilitarias
    // Encontrar el ID de una tarea con su índice en el array
    fn get_index_on_vector(&self, task_id: i32) -> usize {
        let position = 
        match self.task_manager.iter().position(|t| t.get_id() == task_id) {
            Some(pos) => pos,
            None => {
                println!("No se encontró una tarea con el id {task_id}");
                TASK_NULL 
            }
        };
        position
    }



}