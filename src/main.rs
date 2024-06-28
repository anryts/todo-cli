use clap::{Parser, Subcommand, };
use todo::Status;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
        #[clap(short = 's', long, default_value = "todo")]
        status: Status,
        #[clap(short = 'c', long, default_value = "10")]
        count: i32
    },
    Complete {
        id: i32,
    },
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
            => {  
                let items = todo_service.get_todos(status, count);
                let mut stdout = StandardStream::stdout(ColorChoice::Always);
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
                for _val in items.iter() {
                    println!("id: {0} title: {1}\ndescription: {2}\n",
                             _val.id, _val.title, _val.description) 
                }
         
         }
        Commands::Complete {id} => {
            todo_service.change_status(Status::Done, id)
        }
        Commands::Delete => todo!(),
        Commands::Update => todo!(),
        Commands::Search => todo!(),
                    
    }

    println!("\ntotal count: {:?}", todo_service.get_count());
}