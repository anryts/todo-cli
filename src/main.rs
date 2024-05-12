use clap::{Parser, Subcommand, };
use todo::Status;

use crate::todo::Todo;
use crate::todo_service::TodoService;
mod todo_service;
mod todo;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Add {
        #[clap(short = 't', long)]
        title: String,
        #[clap(short = 'd', long)]
        description: String,
    },
    List {
        #[clap(short = 's', long)]
        status: Status,
        #[clap(short = 'c', long)]
        count: i32
    },
    Complete,
    Delete,
    Update,
    Search
}

fn main() {
    let args = Args::parse();
    let mut todo_service = TodoService::new();
  
    match args.cmd {
        Commands::Add {title, description} 
                     => { let item = Todo::new(todo_service.get_count(), title, description);
                        todo_service.add_todo(item);
                    },
        Commands::List {status, count} 
            => {//TODO rewrite this  
                let items = todo_service.get_todos(status, count);
                for _val in items.iter() {
                    println!("id:{0} title:{1}", _val.id, _val.title) 
                }
         
         }
        Commands::Complete => todo!(),
        Commands::Delete => todo!(),
        Commands::Update => todo!(),
        Commands::Search => todo!(),
                    
    }

    println!("{:?}", todo_service.get_count());
}