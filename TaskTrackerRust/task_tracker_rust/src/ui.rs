use crate::manager::TaskManager;
use crate::task::{Status, Task};
use std::io::{self, Write};

pub struct UI<'a> {
    pub manager: &'a mut dyn TaskManager,
}

impl<'a> UI<'a> {
    pub fn new(manager: &'a mut dyn TaskManager) -> Self {
        UI { manager }
    }

    pub fn run(&mut self) {
        println!("Tareas cargadas: {}", self.manager.get_task_count());

        loop {
            self.show_menu();
            let choice = read_int();

            match choice {
                1 => self.handle_list_all(),
                2 => self.handle_add_task(),
                3 => self.handle_update_status(),
                4 => self.handle_delete(),
                5 => self.handle_list_by_status(),
                6 => {
                    println!("¡Hasta luego!");
                    return;
                }
                _ => println!("Opción no válida"),
            }
        }
    }

    fn show_menu(&self) {
        println!("\n=== GESTOR DE TAREAS ===");
        println!("1. Ver todas las tareas");
        println!("2. Agregar nueva tarea");
        println!("3. Cambiar estado de tarea");
        println!("4. Eliminar tarea");
        println!("5. Ver tareas por estado");
        println!("6. Salir");
        print!("Elige una opción: ");
        io::stdout().flush().ok();
    }

    fn print_tasks(&self, tasks: &[Task]) {
        if tasks.is_empty() {
            println!("No hay tareas para mostrar");
            return;
        }

        let mut w_id = "ID".len();
        let mut w_desc = "Descripción".len();
        let mut w_status = "Estado".len();
        let mut w_created = "Creado".len();
        let mut w_updated = "Último Cambio".len();
        let mut w_completed = "Finalizado".len();

        for t in tasks {
            w_id = w_id.max(t.id.to_string().len());
            w_desc = w_desc.max(t.description.len());
            w_status = w_status.max(t.status.to_string().len());
            w_created = w_created.max(Task::format_time(Some(t.created_at)).len());
            w_updated = w_updated.max(Task::format_time(Some(t.updated_at)).len());
            w_completed = w_completed.max(Task::format_time(t.completed_at).len());
        }

        w_id += 2;
        w_desc += 2;
        w_status += 2;
        w_created += 2;
        w_updated += 2;
        w_completed += 2;

        let h = |n: usize| "─".repeat(n);

        println!(
            "\n┌{}┬{}┬{}┬{}┬{}┬{}┐",
            h(w_id), h(w_desc), h(w_status), h(w_created), h(w_updated), h(w_completed)
        );

        println!(
            "│ {:^w_id$} │ {:^w_desc$} │ {:^w_status$} │ {:^w_created$} │ {:^w_updated$} │ {:^w_completed$} │",
            "ID",
            "Descripción",
            "Estado",
            "Creado",
            "Último Cambio",
            "Finalizado",
            w_id = w_id - 2,
            w_desc = w_desc - 2,
            w_status = w_status - 2,
            w_created = w_created - 2,
            w_updated = w_updated - 2,
            w_completed = w_completed - 2
        );

        println!(
            "├{}┼{}┼{}┼{}┼{}┼{}┤",
            h(w_id), h(w_desc), h(w_status), h(w_created), h(w_updated), h(w_completed)
        );

        fn wrap_text(s: &str, width: usize) -> Vec<String> {
            if s.is_empty() {
                return vec!["".to_string()];
            }
            let width = if width == 0 { 1 } else { width };
            let mut lines: Vec<String> = Vec::new();
            let mut current = String::new();
            for word in s.split_whitespace() {
                if current.is_empty() {
                    if word.len() <= width {
                        current.push_str(word);
                    } else {
                        let mut start = 0;
                        let bytes = word.as_bytes();
                        while start < word.len() {
                            let end = (start + width).min(word.len());
                            let part = &word[start..end];
                            lines.push(part.to_string());
                            start = end;
                        }
                    }
                } else {
                    if current.len() + 1 + word.len() <= width {
                        current.push(' ');
                        current.push_str(word);
                    } else {
                        lines.push(current);
                        current = String::new();
                        if word.len() <= width {
                            current.push_str(word);
                        } else {
                            let mut start = 0;
                            while start < word.len() {
                                let end = (start + width).min(word.len());
                                let part = &word[start..end];
                                lines.push(part.to_string());
                                start = end;
                            }
                        }
                    }
                }
            }
            if !current.is_empty() {
                lines.push(current);
            }

            if lines.is_empty() {
                lines.push(String::new());
            }
            lines
        }

        for t in tasks {
            let id_lines = vec![t.id.to_string()];
            let desc_width = w_desc - 2; 
            let desc_lines = wrap_text(&t.description, desc_width);
            let status_lines = vec![t.status.to_string()];
            let created_lines = vec![Task::format_time(Some(t.created_at))];
            let updated_lines = vec![Task::format_time(Some(t.updated_at))];
            let completed_lines = vec![Task::format_time(t.completed_at)];

            let row_lines = *[
                id_lines.len(),
                desc_lines.len(),
                status_lines.len(),
                created_lines.len(),
                updated_lines.len(),
                completed_lines.len(),
            ]
            .iter()
            .max()
            .unwrap_or(&1);

            for i in 0..row_lines {
                let id_cell = if i == 0 { id_lines.get(0).unwrap().as_str() } else { "" };
                let desc_cell = desc_lines.get(i).map(|s| s.as_str()).unwrap_or("");
                let status_cell = status_lines.get(i).map(|s| s.as_str()).unwrap_or("");
                let created_cell = created_lines.get(i).map(|s| s.as_str()).unwrap_or("");
                let updated_cell = updated_lines.get(i).map(|s| s.as_str()).unwrap_or("");
                let completed_cell = completed_lines.get(i).map(|s| s.as_str()).unwrap_or("");

                println!(
                    "│ {:^w_id$} │ {:<w_desc$} │ {:^w_status$} │ {:^w_created$} │ {:^w_updated$} │ {:^w_completed$} │",
                    id_cell,
                    desc_cell,
                    status_cell,
                    created_cell,
                    updated_cell,
                    completed_cell,
                    w_id = w_id - 2,
                    w_desc = w_desc - 2,
                    w_status = w_status - 2,
                    w_created = w_created - 2,
                    w_updated = w_updated - 2,
                    w_completed = w_completed - 2
                );
            }
        }

        println!(
            "└{}┴{}┴{}┴{}┴{}┴{}┘",
            h(w_id), h(w_desc), h(w_status), h(w_created), h(w_updated), h(w_completed)
        );
    }

    fn handle_list_all(&self) {
        let tasks = self.manager.list_all();
        self.print_tasks(&tasks);
    }

    fn handle_add_task(&mut self) {
        print!("Descripción de la tarea: ");
        io::stdout().flush().ok();
        let desc = read_line_trimmed();
        if desc.is_empty() {
            println!("Descripción vacía. Cancelado.");
            return;
        }
        if let Err(e) = self.manager.add(desc) {
            println!("Error al agregar tarea: {}", e);
        } else {
            println!("Tarea agregada exitosamente");
        }
    }

    fn handle_update_status(&mut self) {
        self.handle_list_all();
        print!("ID de la tarea a actualizar: ");
        io::stdout().flush().ok();
        let id = read_int();
        println!("Estados disponibles:\n1. Por hacer\n2. En progreso\n3. Terminada");
        print!("Nuevo estado (1-3): ");
        io::stdout().flush().ok();
        let choice = read_int();
        let status = match choice {
            1 => Status::Todo,
            2 => Status::InProgress,
            3 => Status::Done,
            _ => {
                println!("Opción no válida");
                return;
            }
        };
        if let Err(e) = self.manager.update_status(id, status) {
            println!("Error: {}", e);
        } else {
            println!("Estado actualizado exitosamente");
        }
    }

    fn handle_delete(&mut self) {
        self.handle_list_all();
        print!("ID de la tarea a eliminar: ");
        io::stdout().flush().ok();
        let id = read_int();
        if let Err(e) = self.manager.delete(id) {
            println!("Error: {}", e);
        } else {
            println!("Tarea eliminada exitosamente");
        }
    }

    fn handle_list_by_status(&self) {
        println!("Estados disponibles:\n1. Por hacer\n2. En progreso\n3. Terminada");
        print!("Estado a listar (1-3): ");
        io::stdout().flush().ok();
        let choice = read_int();
        let status = match choice {
            1 => Status::Todo,
            2 => Status::InProgress,
            3 => Status::Done,
            _ => {
                println!("Opción no válida");
                return;
            }
        };
        let tasks = self.manager.list_by_status(status);
        self.print_tasks(&tasks);
    }
}

fn read_line_trimmed() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}

fn read_int() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse::<i32>().unwrap_or(-1)
}
