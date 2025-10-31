use crate::task::Task;
use serde_json;
use std::fs;
use std::io;
use std::path::Path;

pub trait Storage {
    fn load(&self) -> io::Result<Vec<Task>>;
    fn save(&self, tasks: &Vec<Task>) -> io::Result<()>;
}

pub struct JSONStorage {
    pub filename: String,
}

impl JSONStorage {
    pub fn new(filename: &str) -> Self {
        JSONStorage {
            filename: filename.to_string(),
        }
    }
}

impl Storage for JSONStorage {
    fn load(&self) -> io::Result<Vec<Task>> {
        if !Path::new(&self.filename).exists() {
            return Ok(vec![]);
        }
        let data = fs::read_to_string(&self.filename)?;
        let tasks: Vec<Task> = serde_json::from_str(&data)?;
        Ok(tasks)
    }

    fn save(&self, tasks: &Vec<Task>) -> io::Result<()> {
        let data = serde_json::to_string_pretty(tasks)?;
        fs::write(&self.filename, data)?;
        Ok(())
    }
}
