use std::fs;
use crate::todo::{Todo, Status};
use serde_json;

pub struct TodoService {
    items: Vec<Todo>,
}

impl TodoService {
    pub fn new() -> TodoService {
        //read from file
        let items = {
            let res = fs::read_to_string("data.json")
                .expect("Can't read");
            serde_json::from_str::<Vec<Todo>>(&res).unwrap()
        };
        return TodoService {
            items
        };
    }

    pub fn add_todo(&mut self, item: Todo) {
        self.items.push(item);
        //write into json file
        let json_data = serde_json :: to_string_pretty(&self.items).unwrap();
        std::fs::write("data.json", json_data)
            .expect("Can't write data into file");
    }

    pub fn get_todos(&mut self, status: Status, count: i32) -> Vec<Todo> {
        return self.items
        .iter()
        .filter(|todo| todo.status == status)
        .cloned()
        .collect();
    }
    
    pub fn change_status(&mut self, status: Status, id: i32)  {
        //in place mutation be like this 
         if let Some(task) = self.items
            .iter_mut()
            .find(|task| task.id == id) {
            //modify
            task.status = status;
        }
    }

    pub fn get_count(&mut self) -> i32 {
        //read from file and find len()
        let list_todo = {
            let res = fs::read_to_string("data.json")
                .expect("Can't read");
            serde_json::from_str::<Vec<Todo>>(&res).unwrap()
        };
        return list_todo.len() as i32; 
    }
}

