use crate::todo::{Todo, Status};
use serde::{Serialize, Deserialize};
use serde_json::Result;

pub struct TodoService {
    items: Vec<Todo>,
}

impl TodoService {
    pub fn new() -> TodoService {
        return TodoService {
            items: Vec::new()
        };
    }

    pub fn add_todo(&mut self, item: Todo) {
        self.items.push(item);
        //write into json file
        let serialized = serde_json
    }

    pub fn get_todos(&mut self, status: Status, count: i32) -> Vec<Todo> {
        return self.items
        .iter()
        .filter(|todo| todo.status == status)
        .cloned()
        .collect();
    }

    pub fn get_count(&mut self) -> i32 {
        return self.items.len() as i32; 
    }
}

