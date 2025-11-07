use anyhow::{Error, Result};
use colored::Colorize;
use sqlx::{Pool, Sqlite, prelude::FromRow};
use std::sync::Arc;

use crate::helpers::{get_timestamp, readable_datetime};

#[derive(Debug, Clone, FromRow)]
pub struct Todo {
    id: i32,
    message: String,
    is_done: bool,
    created_at: u64,
}

pub struct TodoOptions;

impl TodoOptions {
    pub async fn select_action_by_option(
        option_name: &str,
        db_pool: Arc<Pool<Sqlite>>,
        args: &[String],
    ) -> Result<()> {
        match option_name {
            "list" => TodoOptions::todo_list(db_pool).await?,
            "add" => TodoOptions::add_todo(db_pool, args).await?,
            "done" => TodoOptions::done_todo(db_pool, args).await?,
            "delete" => TodoOptions::delete_todo(db_pool, args).await?,
            _ => (),
        }

        Ok(())
    }

    async fn todo_list(db_pool: Arc<Pool<Sqlite>>) -> Result<()> {
        let todos: Vec<Todo> = sqlx::query_as("SELECT * FROM todo")
            .fetch_all(&*db_pool)
            .await?;

        if todos.is_empty() {
            println!("📭  The list is empty. Try adding a new task!");
            return Ok(());
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
    async fn add_todo(db_pool: Arc<Pool<Sqlite>>, args: &[String]) -> Result<()> {
        if args.is_empty() {
            return Err(Error::msg(
                "Please add todo message like: dojo add task1".bold().red(),
            ));
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
    async fn done_todo(db_pool: Arc<Pool<Sqlite>>, args: &[String]) -> Result<()> {
        if args.is_empty() {
            return Err(Error::msg(
                "Please add todos id like: dojo done 1".bold().red(),
            ));
        }

        for arg in args {
            // update db
            let parsed_todo_id = match arg.parse::<i32>() {
                Ok(v) => v,
                Err(_) => {
                    println!(
                        "{}",
                        String::from("error: done operation args most be i32. like: dojo done 1")
                            .bold()
                            .red()
                    );
                    continue;
                }
            };
            sqlx::query("UPDATE todo SET is_done = 1 WHERE id = $1")
                .bind(parsed_todo_id)
                .execute(&*db_pool)
                .await?;
        }

        println!("{}", String::from("successfully updated").bold().green());
        Ok(())
    }
    async fn delete_todo(db_pool: Arc<Pool<Sqlite>>, args: &[String]) -> Result<()> {
        if args.is_empty() {
            return Err(Error::msg(
                "Please add todos id like: dojo delete 1".bold().red(),
            ));
        }

        for arg in args {
            // delete from db
            let parsed_todo_id = match arg.parse::<i32>() {
                Ok(v) => v,
                Err(_) => {
                    println!(
                        "{}",
                        String::from(
                            "error: delete operation args most be i32. like: dojo delete 1"
                        )
                        .bold()
                        .red()
                    );
                    continue;
                }
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use dotenvy::dotenv;

    use crate::{
        helpers::{get_timestamp, init_db_pool},
        models::{Todo, TodoOptions},
    };

    #[tokio::test]
    async fn test_list_action() {
        dotenv().ok();
        let db_pool = Arc::new(init_db_pool(true).await.unwrap());

        assert!(TodoOptions::todo_list(db_pool).await.is_ok());
    }

    #[tokio::test]
    async fn test_add_action() {
        dotenv().ok();
        let db_pool = Arc::new(init_db_pool(true).await.unwrap());

        // create new todo and assert it
        assert!(
            TodoOptions::add_todo(db_pool.clone(), &[String::from("test")])
                .await
                .is_ok()
        );

        // fetch last todo and check message is test or no
        let last_todo_from_db: Todo =
            sqlx::query_as("SELECT * FROM todo ORDER BY ROWID DESC LIMIT 1")
                .fetch_one(&*db_pool)
                .await
                .unwrap();

        assert_eq!(last_todo_from_db.message, "test");
    }

    #[tokio::test]
    async fn test_done_action() {
        dotenv().ok();
        let db_pool = Arc::new(init_db_pool(true).await.unwrap());

        // create a todo and return id
        let c_todo = sqlx::query_as::<_, (i32,)>(
            "INSERT INTO todo (message, is_done, created_at) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind("hello")
        .bind(0)
        .bind(get_timestamp() as i32)
        .fetch_one(&*db_pool)
        .await
        .unwrap();

        // run done_todo for created todo
        TodoOptions::done_todo(db_pool.clone(), &[c_todo.0.to_string()])
            .await
            .unwrap();

        // fetch created todo and check is_done eq true or no
        let fetch: Todo = sqlx::query_as("SELECT * FROM todo WHERE id = $1")
            .bind(c_todo.0)
            .fetch_one(&*db_pool)
            .await
            .unwrap();

        assert_eq!(fetch.is_done, true);
    }

    #[tokio::test]
    async fn test_add_todo_empty_args_error() {
        dotenv().ok();
        let db_pool = Arc::new(init_db_pool(true).await.unwrap());

        assert!(TodoOptions::add_todo(db_pool.clone(), &[]).await.is_err());
    }

    #[tokio::test]
    async fn test_done_todo_empty_args_error() {
        dotenv().ok();
        let db_pool = Arc::new(init_db_pool(true).await.unwrap());

        assert!(TodoOptions::done_todo(db_pool.clone(), &[]).await.is_err())
    }

    #[tokio::test]
    async fn test_delete_todo() {
        dotenv().ok();
        let db_pool = Arc::new(init_db_pool(true).await.unwrap());

        // create a todo and return id
        let c_todo = sqlx::query_as::<_, (i32,)>(
            "INSERT INTO todo (message, is_done, created_at) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind("hello")
        .bind(0)
        .bind(get_timestamp() as i32)
        .fetch_one(&*db_pool)
        .await
        .unwrap();

        // run delete_todo for created todo
        TodoOptions::delete_todo(db_pool.clone(), &[c_todo.0.to_string()])
            .await
            .unwrap();

        // fetch created todo and check is exist or no
        let fetch = sqlx::query_as::<_, Todo>("SELECT * FROM todo WHERE id = $1")
            .bind(c_todo.0)
            .fetch_one(&*db_pool)
            .await;

        assert!(fetch.is_err())
    }

    #[tokio::test]
    async fn test_delete_todo_empty_args_error() {
        dotenv().ok();
        let db_pool = Arc::new(init_db_pool(true).await.unwrap());

        assert!(
            TodoOptions::delete_todo(db_pool.clone(), &[])
                .await
                .is_err()
        )
    }
}
