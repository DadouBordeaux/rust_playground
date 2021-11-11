use std::fs::{read_to_string, File};
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let todo_list_name: &'static str = "ma todo list";

    let todo_list: TodoList = TodoList::new(todo_list_name);

    println!("{:?} after create todo list", todo_list);

    //TODO READ https://doc.rust-lang.org/rust-by-example/conversion/from_into.html#from
    //TODO READ https://doc.rust-lang.org/std/convert/trait.From.html
    // impl From<ReadDir> for Directory {
    //     fn from(read_dir: ReadDir) -> Self {
    //         Directory::new(
    //             read_dir
    //                 .map(|dir_entry_result| {
    //                     let dir_entry = dir_entry_result.unwrap(); //TODO READ https://doc.rust-lang.org/std/result/index.html
    //                     let name = dir_entry.file_name().into_string().unwrap();
    //                     let metadata = dir_entry.metadata().unwrap();

    //                     if metadata.is_dir() {
    //                         Node::new_directory(&name)
    //                     } else {
    //                         Node::new_file(&name)
    //                     }
    //                 })
    //                 .collect(),
    //         )
    //     }
    // }

    save_todo_list(&todo_list)?;

    let todo_todo_list: Todo = Todo::new(
        String::from("Make a todo list"),
        String::from("First try to do a todo list in rust"),
    );

    let new_todo_list = todo_list.add_a_todo(&todo_todo_list);
    // println!("{:?} added new todo to todolist", new_todo_list);

    let found_todo = new_todo_list.get_todo("Make a todo list").unwrap();

    let mut new_found_todo = found_todo.clone();
    new_found_todo.set_complete();

    let mut todo_list_with_complete = new_todo_list.add_a_todo(&new_found_todo);

    let all_todo_complete = todo_list_with_complete.validate_all_todo();

    println!("{:?} all_todo_complete", all_todo_complete);
    save_todo_list(&todo_list_with_complete)?;

    Ok(())
}

#[derive(Debug)]
struct TodoList {
    name: String,
    todos: Vec<Todo>,
}

impl TodoList {
    pub fn new(name: &str) -> Self {
        TodoList {
            name: name.to_string(),
            todos: vec![],
        }
    }

    pub fn get_todo(&self, name: &str) -> Result<&Todo, String> {
        match self.todos.iter().find(|&e| e.name == name) {
            Some(found_todo) => Ok(found_todo),
            None => Err(String::from("ERR")),
        }
    }

    pub fn add_a_todo(self, todo: &Todo) -> Self {
        let mut new_todos = self.todos.clone();
        new_todos.push(todo.clone());
        TodoList {
            name: self.name,
            todos: new_todos,
        }
    }

    pub fn add_todo(&mut self, todo: Todo) {
        //AXM => Access Xor Mutability
        self.todos.push(todo)
    }

    pub fn validate_all_todo(&mut self) -> TodoList {
        let new_todos = self
            .todos
            .iter()
            .map(|todo| -> Todo {
                let mut todo_to_complete = todo.clone();
                todo_to_complete.set_complete();

                todo_to_complete
            })
            .collect();

        TodoList {
            name: self.name.clone(),
            todos: new_todos,
        }
    }
}

#[derive(Clone, Debug)]
enum TodoState {
    Complete,
    Todo,
}

#[derive(Clone, Debug)]
struct Todo {
    name: String,
    description: String,
    state: TodoState,
}

impl Todo {
    pub fn new(name: String, description: String) -> Todo {
        Todo {
            name,
            description: description,
            state: TodoState::Todo,
        }
    }

    pub fn set_complete(&mut self) {
        self.state = TodoState::Complete;
    }
}

fn save_todo_list(todo_list: &TodoList) -> Result<(), std::io::Error> {
    let mut file = File::create(format!("{}.json", todo_list.name))?;

    file.write_all(b"")?;

    Ok(())
}

fn update_todo_list(todo_list: &TodoList, todo: Todo) -> Result<(), std::io::Error> {
    let path = &todo_list.name;
    let mut file = File::open(path);

    // let mut file = File::create(format!("{}.json", todo_list.name))?;
    // file.write_all(b"")?;

    Ok(())
}
