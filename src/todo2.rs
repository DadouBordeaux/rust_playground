mod index;

#[derive(Debug)]
pub enum State {
    Draft,
    Validated,
    Complete,
}
#[derive(Debug)]
pub struct TodoItem {
    //TODO date (chrono) & tasks
    name: String,
    state: State,
    description: String,
}
// https://doc.rust-lang.org/book/ch04-03-slices.html
// a bucher : https://doc.rust-lang.org/std/ops/trait.Deref.html
// une slice EST un smart pointer : https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
// https://medium.com/the-polyglot-programmer/undestanding-rust-smart-pointers-660d59715ab9

impl TodoItem {
    pub fn new(name: &str, description: &str) -> Self {
        let s = String::new();

        //str::replace // et pas String::replace
        //s.replace(...);
        //(&s).replace(...); //exemple de coercion

        TodoItem {
            name: name.to_string(),
            state: State::Draft,
            description: description.to_string(),
        }
    }

    pub fn validate(&mut self) {
        self.state = State::Validated;
    }

    pub fn complete(&mut self) {
        self.state = State::Complete;
    }

    pub fn hey(&mut self) -> &str {
        self.name.to_string()
    }

    pub fn as_name(&self) -> &str {
        &self.name
    }

    pub fn as_state(&self) -> &State {
        &self.state
    }

    pub fn to_name(&self) -> String {
        self.name.to_string()
    }

    pub fn as_description(&self) -> &String {
        &self.description
    }

    pub fn to_description(&self) -> String {
        self.description.to_string()
    }
}

#[derive(Debug)]
pub struct TodoList(pub Vec<TodoItem>); //Tuple struct

impl TodoList {
    pub fn new(list: Vec<TodoItem>) -> Self {
        TodoList(list)
    }

    pub fn add(&mut self, todo: TodoItem) -> Result<(), String> {
        // if (todo.to_name() === )
        // iter().find(|| predicate).is_some()
        let item_found = self
            .0
            .iter()
            .any(|todo_item| todo_item.to_name() == todo.to_name());
        if item_found {
            Err("Name already taken".to_string())
        } else {
            self.0.push(todo);
            Ok(())
        }
    }
}

impl Default for TodoList {
    fn default() -> Self {
        TodoList::new(Vec::new())
    }
}

fn main() -> Result<(), String> {
    let todo = TodoItem::new("do the todo", "This is my first toto item");
    /* let todo2 = TodoItem::new("do the todo", "This is my first toto item"); */
    let mut todo_list = TodoList::default();

    let name = todo.to_name();
    todo_list.add(todo)?;
    /*
    match todo_list.add(todo2) {
        Ok(_) => {},
        Err(err) => return Err(err)
    }
    */

    println!("{}", name);
    println!("{:#?}", todo_list);

    Ok(())
}
