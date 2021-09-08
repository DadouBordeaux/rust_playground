fn main() {
    let mut todos: Todos = vec![];

    let todo_to_complete: Todo = create_a_todo(String::from("1"), String::from("Make a todo list"));

    todos.push(todo_to_complete);

    let found_todo = get_todo("1", &todos).unwrap();

    let todo_completed: Todo = complete_todo(found_todo);
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

fn get_todo<'a>(id: &str, todos: &'a Todos) -> Result<&'a Todo, String> {
    match todos.iter().find(|&e| e.id == id) {
        Some(found_todo) => Ok(found_todo),
        None => Err(String::from("ERR")),
    }
}
