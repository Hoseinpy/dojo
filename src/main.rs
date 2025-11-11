use std::{env, sync::Arc};

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
    let db_pool = Arc::new(
        init_db_pool(false)
            .await
            .unwrap_or_else(|_| panic!("{}", "failed to create db pool".bold().red())),
    );

    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!(
            "{}",
            String::from("invalid usage. run 'dojo help' for help")
                .bold()
                .red()
        );
        return;
    }
    match args[0].as_str() {
        "add" => {
            if let Err(e) = TodoOptions::add_todo(db_pool.clone(), &args[1..]).await {
                println!(
                    "{}",
                    format!("failed to handle add. error: {e:?}").bold().red()
                );
            }
        }
        "done" => {
            if let Err(e) = TodoOptions::done_todo(db_pool.clone(), &args[1..]).await {
                println!(
                    "{}",
                    format!("failed to handle done. error: {e:?}").bold().red()
                );
            }
        }
        "list" => {
            if let Err(e) = TodoOptions::todo_list(db_pool.clone()).await {
                println!(
                    "{}",
                    format!("failed to handle list. error: {e:?}").bold().red()
                );
            }
        }
        "delete" => {
            if let Err(e) = TodoOptions::delete_todo(db_pool.clone(), &args[1..]).await {
                println!(
                    "{}",
                    format!("failed to handle delete. error: {e:?}")
                        .bold()
                        .red()
                );
            }
        }
        "help" | "-h" | "--help" => print_help(),
        "version" => {
            let version = get_version();
            println!("Version: {}", version)
        }
        _ => (),
    }

    db_pool.close().await;
}
