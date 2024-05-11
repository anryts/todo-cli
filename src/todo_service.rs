use crate::todo::{Todo, Status};

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
        self.items.push(item)
    }

    pub fn get_count(&mut self) -> i32 {
        return self.items.len() as i32; 
    }
}

