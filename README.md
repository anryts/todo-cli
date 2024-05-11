My plan is to create simple todo app but in console.
i want to use rust for this project.
to store data, i will just use json file.
example of this json file:

```json
{
  "todos": [
    {
      "id": 1,
      "title": "Learn Rust",
      "description": "Learn rust programming language",
      "status": "incomplete"
    },
    {
      "id": 2,
      "title": "Learn JavaScript",
      "description": "Learn go programming language",
      "status": "incomplete"
    }
  ]
}
```

so, it will have todos array, and each todo will have id, title, description, and status.

i will create simple command line interface to interact with this json file.
the commands will be:

- add todo
- list todos
- complete todo
- delete todo
- update todo
- search todo
- help

i want to create more user friendly interface, so i will use [dialoguer](https://docs.rs/dialoguer/0.7.1/dialoguer/) crate to create interactive command line interface.

my checklist for this project:

- [x] create todo struct
- [x] create todo status enum
- [ ] create todo service
  - [x] add todo
  - [ ] list todos
  - [ ] complete todo
  - [ ] delete todo
  - [ ] update todo
  - [ ] search todo
- [ ] create command line interface
  - [x] add todo - bash command: `todo add --title Learn Rust --description Learn rust programming language`
  - [ ] list todos - bash command: `todo list --status "incomplete"`
  - [ ] complete todo - bash command: `todo complete --id 1`
  - [ ] delete todo - bash command: `todo delete --id 1`
  - [ ] update todo - bash command: `todo update --id 1 --title "Learn Rust" --description "Learn rust programming language"`
  - [ ] search todo - bash command: `todo search --title "Learn Rust"`
  - [ ] help - bash command: `todo help`
- [ ] create json file
- [ ] read json file
- [ ] write json file
- [ ] add help command

## CLI commands

i will call it from terminal like this:

```bash
$ todo add --title "Learn Rust" --description "Learn rust programming language"
$ todo list --status "incomplete"
$ todo complete --id 1
$ todo delete --id 1
$ todo update --id 1 --title "Learn Rust" --description "Learn rust programming language"
$ todo search --title "Learn Rust"
$ todo help
```

## in future, need to do more fancy TUI
