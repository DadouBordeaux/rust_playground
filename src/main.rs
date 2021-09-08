fn main() {
    let mut todos: Todos = vec![];

    let todo_to_complete: Todo = create_a_todo(String::from("1"), String::from("Make a todo list"));

    todos.push(todo_to_complete);

    // Retrieve todo with match
    let found_todo_match = todos.iter().find(|&e| e.id == "1");
    println!("{:?}", found_todo_match);
    match found_todo_match {
        Some(found_todo_match) => {
            let todo_completed: Todo = complete_todo(found_todo_match);
            println!("{}", todo_completed.description);
            println!("{:?}", todo_completed.state);
        }
        None => { /* We didn't find one */ }
    };

    // Retrieve todo with unwrap
    let found_todo_unwrap = todos.iter().find(|&e| e.id == "1").unwrap();
    println!("{:?}", found_todo_unwrap);

    let todo_completed: Todo = complete_todo(found_todo_unwrap);
    println!("{}", todo_completed.description);
    println!("{:?}", todo_completed.state);
}

type Todos = Vec<Todo>;

#[derive(Debug)]
enum TodoState {
    Complete,
    Todo,
}

#[derive(Debug)]
struct Todo {
    id: String,
    description: String,
    state: TodoState,
}

fn create_a_todo(id: String, description: String) -> Todo {
    Todo {
        id: id,
        description: description,
        state: TodoState::Todo,
    }
}

fn complete_todo(todo: &Todo) -> Todo {
    Todo {
        id: todo.id.to_owned(),
        description: todo.description.to_owned(),
        state: TodoState::Complete,
    }
}
