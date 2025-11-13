use anyhow::{Error, Ok, Result};
use chrono::{Local, TimeZone};
use colored::{ColoredString, Colorize};
use sqlx::{Pool, Sqlite, migrate::MigrateDatabase, sqlite::SqlitePoolOptions};
use std::{
    env::{self},
    fs,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub fn get_timestamp() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH).ok() {
        Some(v) => v.as_secs(),
        None => 0,
    }
}

pub async fn init_db_pool(test_mode: bool) -> Result<Pool<Sqlite>> {
    // set variables
    let database_url = if test_mode {
        format!(
            "sqlite:file:memdb_{}?mode=memory&cache=shared",
            uuid::Uuid::new_v4()
        )
    } else {
        env::var("DATABASE_URL")?
    };

    // create dir /db if not in test mode
    if !test_mode {
        fs::create_dir_all("db/")?;
    }

    // create database if not exists
    if !test_mode && !Sqlite::database_exists(&database_url).await? {
        Sqlite::create_database(&database_url).await?;
    }

    // create pool
    let pool = SqlitePoolOptions::new()
        .idle_timeout(Duration::from_secs(10))
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .max_lifetime(Duration::from_secs(30))
        .connect(&database_url)
        .await?;

    // run migrations
    let _ = sqlx::migrate!("./migrations").run(&pool).await;

    Ok(pool)
}

pub fn readable_datetime(timestamp: u64) -> Result<String> {
    let datetime = match Local.timestamp_opt(timestamp as i64, 0).single() {
        Some(v) => v,
        None => {
            return Err(Error::msg("datetime is none"));
        }
    };
    Ok(datetime.to_rfc2822())
}

pub fn print_help_text(command_name: ColoredString, description: &str, usage: String) {
    println!(
        "{}\n  {} {}\n  {} {}\n",
        command_name,
        "Usage:".bold(),
        usage,
        "Description:".bold(),
        description
    );
}

pub fn print_help() {
    println!(
        "\n{}\n{}\n",
        "🧭 AVAILABLE COMMANDS:".bold().cyan(),
        "──────────────────────────────────────────────".dimmed()
    );

    // list command
    print_help_text(
        String::from("List").green().bold().underline(),
        "show available todos",
        String::from("dojo list"),
    );

    // add command
    print_help_text(
        String::from("Add").green().bold().underline(),
        "add new todo",
        String::from("dojo add <message>"),
    );

    // done command
    print_help_text(
        String::from("Done").green().bold().underline(),
        "mark todo as done",
        String::from("dojo done <ids>"),
    );

    // delete command
    print_help_text(
        String::from("Delete").green().bold().underline(),
        "remove todo",
        String::from("dojo delete <ids>"),
    );

    // version command
    print_help_text(
        String::from("Version").green().bold().underline(),
        "show program version",
        String::from("dojo version"),
    );
}

pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
