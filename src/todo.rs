struct Todo {
    id: i32,
    title: String,
    description: String,
    status: Status 
}

impl Todo {
    fn new (id: i32,
            title: String,
            desciption: String) -> Todo {
        return Todo {
            id: id,
            title: title,
            description: desciption,
            status: Status::Incomplete
        };
    }
}