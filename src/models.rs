use anyhow::{Error, Result};
use colored::Colorize;
use sqlx::{Pool, Sqlite, prelude::FromRow};
use std::sync::Arc;

use crate::helpers::{get_timestamp, readable_datetime};

#[derive(Debug, Clone, FromRow)]
pub struct Todo {
    pub id: i32,
    pub message: String,
    pub is_done: bool,
    pub created_at: u64,
}

pub struct TodoOptions;

impl TodoOptions {
    pub async fn todo_list(db_pool: Arc<Pool<Sqlite>>) -> Result<()> {
        let todos: Vec<Todo> = sqlx::query_as("SELECT * FROM todo")
            .fetch_all(&*db_pool)
            .await?;

        if todos.is_empty() {
            return Err(Error::msg("📭  The list is empty. Try adding a new task!"));
        }

        println!("\n{}", "📝 TODO LIST".bold().underline().cyan());
        println!(
            "{}",
            "──────────────────────────────────────────────".dimmed()
        );

        for todo in todos.iter() {
            let status = if todo.is_done {
                "✅ Completed".green()
            } else {
                "⭕ Not Completed".red().bold()
            };

            println!(
                "{} {}\n{} {}\n{} {}\n{} {}\n{}",
                "🆔".bold(),
                todo.id.to_string().bold(),
                "📌 Task:".bold(),
                todo.message,
                "📅 Created at:".bold(),
                readable_datetime(todo.created_at)?,
                "📊 Status:".bold(),
                status,
                "──────────────────────────────────────────────".dimmed()
            );
        }

        Ok(())
    }
    pub async fn add_todo(db_pool: Arc<Pool<Sqlite>>, args: &[String]) -> Result<()> {
        if args.is_empty() {
            return Err(Error::msg("Please add todo message like: dojo add task1"));
        }
        let message = args.join(" ").trim().to_string();

        // add to db
        let created_at = get_timestamp();
        sqlx::query("INSERT INTO todo (message, is_done, created_at) VALUES ($1, $2, $3)")
            .bind(message)
            .bind(0)
            .bind(created_at as i32)
            .execute(&*db_pool)
            .await?;

        println!("{}", String::from("successfully added").bold().green());
        Ok(())
    }
    pub async fn done_todo(db_pool: Arc<Pool<Sqlite>>, args: &[String]) -> Result<()> {
        if args.is_empty() {
            return Err(Error::msg("Please add todos id like: dojo done 1"));
        }

        for arg in args {
            // update db
            let parsed_todo_id = match arg.parse::<i32>() {
                Ok(v) => v,
                Err(_) => continue,
            };
            sqlx::query("UPDATE todo SET is_done = 1 WHERE id = $1")
                .bind(parsed_todo_id)
                .execute(&*db_pool)
                .await?;
        }

        println!("{}", String::from("successfully updated").bold().green());
        Ok(())
    }
    pub async fn delete_todo(db_pool: Arc<Pool<Sqlite>>, args: &[String]) -> Result<()> {
        if args.is_empty() {
            return Err(Error::msg("Please add todos id like: dojo delete 1"));
        }

        for arg in args {
            // delete from db
            let parsed_todo_id = match arg.parse::<i32>() {
                Ok(v) => v,
                Err(_) => continue,
            };
            sqlx::query("DELETE FROM todo WHERE id = $1")
                .bind(parsed_todo_id)
                .execute(&*db_pool)
                .await?;
        }

        println!("{}", String::from("successfully deleted").bold().green());
        Ok(())
    }
}
