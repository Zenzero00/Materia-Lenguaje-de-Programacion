mod manager;
mod storage;
mod task;
mod ui;

use manager::{TaskManagerImpl, TaskManager};
use storage::JSONStorage;
use ui::UI;

fn main() {
    let storage = JSONStorage::new("tasks.json");
    let mut manager_impl = match TaskManagerImpl::new(&storage) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error al inicializar el gestor de tareas: {}", e);
            return;
        }
    };

    let manager_trait_obj: &mut dyn TaskManager = &mut manager_impl;
    let mut ui = UI::new(manager_trait_obj);
    ui.run();
}