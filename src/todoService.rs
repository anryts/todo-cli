struct TodoService {
    items: Vec<Todo>
}

impl TodoService {
    fn new() -> TodoService {
        return TodoService{ 
            items: Vec::new()
        };
    }
}