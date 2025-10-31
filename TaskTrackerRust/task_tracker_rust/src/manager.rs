use crate::storage::Storage;
use crate::task::{Status, Task};
use std::io;

pub trait TaskManager {
    fn add(&mut self, description: String) -> io::Result<()>;
    fn update_status(&mut self, id: i32, status: Status) -> io::Result<()>;
    fn delete(&mut self, id: i32) -> io::Result<()>;
    fn list_all(&self) -> Vec<Task>;
    fn list_by_status(&self, status: Status) -> Vec<Task>;
    fn get_task_count(&self) -> usize;
}

pub struct TaskManagerImpl<'a> {
    pub storage: &'a dyn Storage,
    pub tasks: Vec<Task>,
    next_id: i32,
}

impl<'a> TaskManagerImpl<'a> {
    pub fn new(storage: &'a dyn Storage) -> io::Result<Self> {
        let tasks = storage.load()?;
        let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        Ok(TaskManagerImpl {
            storage,
            tasks,
            next_id,
        })
    }

    fn persist(&self) -> io::Result<()> {
        self.storage.save(&self.tasks)
    }
}

impl<'a> TaskManager for TaskManagerImpl<'a> {
    fn add(&mut self, description: String) -> io::Result<()> {
        let now = Task::now_timestamp();
        let task = Task {
            id: self.next_id,
            description,
            status: Status::Todo,
            created_at: now,
            updated_at: now,
            completed_at: None,
        };
        self.tasks.push(task);
        self.next_id += 1;
        self.persist()
    }

    fn update_status(&mut self, id: i32, status: Status) -> io::Result<()> {
        let now = Task::now_timestamp();
        for t in self.tasks.iter_mut() {
            if t.id == id {
                let was_done = t.status == Status::Done;
                t.status = status.clone();
                t.updated_at = now;
                if status == Status::Done && !was_done {
                    t.completed_at = Some(now);
                } else if status != Status::Done {
                    t.completed_at = None;
                }
                return self.persist();
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "Tarea no encontrada"))
    }

    fn delete(&mut self, id: i32) -> io::Result<()> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            return self.persist();
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "Tarea no encontrada"))
    }

    fn list_all(&self) -> Vec<Task> {
        self.tasks.clone()
    }

    fn list_by_status(&self, status: Status) -> Vec<Task> {
        self.tasks
            .iter()
            .filter(|t| t.status == status)
            .cloned()
            .collect()
    }

    fn get_task_count(&self) -> usize {
        self.tasks.len()
    }
}
