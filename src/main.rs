use std::sync::{Arc, Mutex};

use crate::helpers::TodoOptions;

mod helpers;

fn main() {
    let todos = Arc::new(Mutex::new(Vec::new()));
    println!("hello welcome to simple todo list\n");

    loop {
        println!("1 - new todo | 2 - done todo | 3 - todo list | 4 - quit. choice:");

        let mut user_choice = String::new();
        let parsed_user_choice = match std::io::stdin().read_line(&mut user_choice) {
            Ok(_) => user_choice.trim().parse::<u8>().unwrap(),
            Err(e) => {
                println!("failed to read line. error: {e:?}");
                break;
            }
        };

        if parsed_user_choice < 1 || parsed_user_choice > 4 {
            println!("invalid input");
            continue;
        }

        match parsed_user_choice {
            1 => {
                TodoOptions::select_action_by_option("add", todos.clone());
            }
            2 => {
                TodoOptions::select_action_by_option("done", todos.clone());
            }
            3 => {
                TodoOptions::select_action_by_option("list", todos.clone());
            }
            4 => {
                println!("thanks for using");
                break;
            }
            _ => break,
        }
    }
}
