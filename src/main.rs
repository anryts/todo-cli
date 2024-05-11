use clap::{Parser, Subcommand};

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
        #[clap(short, long)]
        title: String,
        #[clap(short, long)]
        description: String,
    },
    List,
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
        Commands::List => todo!(),
        Commands::Complete => todo!(),
        Commands::Delete => todo!(),
        Commands::Update => todo!(),
        Commands::Search => todo!(),
                    
    }

    println!("{:?}", todo_service.get_count());
}