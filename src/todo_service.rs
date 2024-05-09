use crate::todo::{Todo, Status};

struct TodoService {
    items: Vec<Todo>,
}

impl TodoService {
    fn new() -> TodoService {
        return TodoService {
            items: Vec::new()
        };
    }

    fn add_todo(&mut self, item: Todo) {
        self.items.push(item)
    }
}

