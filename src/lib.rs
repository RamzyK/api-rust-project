use std::collections::HashMap;
use std::result::Result;

pub struct Task {
    pub text: String,

    pub done: bool,
}

pub struct TaskManager {
    tasks: HashMap<usize, Task>,
    next_id: usize,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: HashMap::new(), next_id: 0 }
    }

    pub fn all(&self) -> &HashMap<usize, Task> {
        &self.tasks
    }

    pub fn add(&mut self, text: Option<String>) -> usize {
        let id = self.next_id;
        let task = Task { text: text.unwrap(), done: false };
        if self.tasks.insert(id, task).is_some() {
            panic!("Overwrote task; did next_id wrap?");
        }
        self.next_id += 1;
        id
    }

    pub fn get(&mut self, id: usize) -> Result<&Task, &'static str> {
        self.tasks.get(&id).ok_or("No such task")
    }

    pub fn set(&mut self, id: usize, text: Option<String>, done: Option<bool>)
               -> Result<(), &'static str> {

        self.tasks.get_mut(&id).map_or(
            Err("No such task"),
            |v| {
                if let Some(text) = text {
                    v.text = text;
                }
                if let Some(done) = done {
                    v.done = done;
                }
                Ok(())
            })
            
    }

    pub fn delete(&mut self, id: usize) -> Result<(), &'static str> {
        self.tasks.remove(&id).map_or(Err("No such task"), |_| Ok(()))
    }
}