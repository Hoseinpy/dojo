use std::{
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[derive(Debug, Clone)]
pub struct Todo {
    id: u64,
    message: String,
    is_done: bool,
    created_at: u64,
}

pub struct TodoOptions;

impl TodoOptions {
    pub fn select_action_by_option(option_name: &str, todos: Arc<Mutex<Vec<Todo>>>) {
        match option_name {
            "list" => TodoOptions::todo_list(todos),
            "add" => TodoOptions::add_todo(todos),
            "done" => TodoOptions::done_todo(todos),
            _ => (),
        }
    }

    fn todo_list(todos: Arc<Mutex<Vec<Todo>>>) {
        let todos = todos.lock().unwrap();
        if todos.is_empty() {
            println!("list was empty");
            return;
        }
        for todo in todos.iter() {
            println!(
                "----------\n{}\ntask: {}\nis_done: {}\ncreated_at: {}\n----------",
                todo.id, todo.message, todo.is_done, todo.created_at
            )
        }
    }
    fn add_todo(todos: Arc<Mutex<Vec<Todo>>>) {
        let mut todos = todos.lock().unwrap();
        println!("give me todo message:");
        let mut todo_message_input = String::new();
        std::io::stdin()
            .read_line(&mut todo_message_input)
            .expect("failed to read line");

        let id = match todos.last() {
            Some(v) => v.id + 1,
            None => 1,
        };
        let todo = Todo {
            id: id,
            message: todo_message_input.trim().to_string(),
            is_done: false,
            created_at: get_timestamp(),
        };
        todos.push(todo);

        println!("successfully todo added");
        return;
    }
    fn done_todo(todos: Arc<Mutex<Vec<Todo>>>) {
        let mut todos = todos.lock().unwrap();
        println!("give me todo id:");
        let mut todo_id_input = String::new();
        let parsed_todo_id_input = match std::io::stdin().read_line(&mut todo_id_input) {
            Ok(_) => todo_id_input.trim().parse::<u64>().unwrap(),
            Err(e) => {
                println!("failed to read line. error: {e:?}");
                return;
            }
        };

        for todo in todos.iter_mut() {
            if todo.id == parsed_todo_id_input {
                todo.is_done = true;
                println!("successfully marked as read");
                return;
            }
        }

        println!("cannot found entered todo id!!");
        return;
    }
}
