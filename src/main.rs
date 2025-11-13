use std::{env, process, sync::Arc};

use crate::{
    helpers::{get_version, init_db_pool, print_help},
    models::TodoOptions,
};
use colored::Colorize;
use dotenvy::dotenv;

mod helpers;
mod models;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    // init dotenv
    dotenv().ok();

    // init database pool
    let db_pool = Arc::new(init_db_pool(false).await.unwrap_or_else(|_| {
        eprintln!("{}", String::from("failed to create db pool").red().bold());
        process::exit(1)
    }));

    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!(
            "{}",
            String::from("invalid usage. run 'dojo help'").bold().red()
        );
        process::exit(1)
    }

    match args[0].as_str() {
        "add" => {
            if let Err(e) = TodoOptions::add_todo(db_pool.clone(), &args[1..]).await {
                eprintln!("{}", format!("{e:?}").bold().red());
            }
        }
        "done" => {
            if let Err(e) = TodoOptions::done_todo(db_pool.clone(), &args[1..]).await {
                eprintln!("{}", format!("{e:?}").bold().red());
            }
        }
        "list" => {
            if let Err(e) = TodoOptions::todo_list(db_pool.clone()).await {
                eprintln!("{}", format!("{e:?}").bold().red());
            }
        }
        "delete" => {
            if let Err(e) = TodoOptions::delete_todo(db_pool.clone(), &args[1..]).await {
                eprintln!("{}", format!("{e:?}").bold().red());
            }
        }
        "version" => {
            let version = get_version();
            println!("Version: {}", version)
        }
        _ => print_help(),
    }

    db_pool.close().await;
}
