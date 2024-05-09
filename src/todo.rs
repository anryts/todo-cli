pub struct Todo {
    id: i32,
    title: String,
    description: String,
    status: Status 
}

impl Todo {
    fn new (id: i32,
            title: String,
            description: String) -> Todo {
        return Todo {
            id,
            title,
            description,
            status: Status::Incomplete
        };
    }
}

pub enum Status {
    Complete,
    InProgress,
    Incomplete
}