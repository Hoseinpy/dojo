use crate::helpers::{
    get_timestamp, get_version, init_db_pool, print_help_text, readable_datetime,
};
use colored::Colorize;
use dotenvy::dotenv;

#[test]
fn test_get_timestamp() {
    assert!(get_timestamp() > 0)
}

#[test]
fn test_readable_datetime() {
    let timestamp = get_timestamp();
    assert!(readable_datetime(timestamp).is_ok())
}

#[test]
fn test_get_help_text() {
    print_help_text(
        String::from("test").underline(),
        "test desc",
        String::from("dojo test"),
    );
}

#[test]
fn test_get_version() {
    assert!(!get_version().is_empty())
}

#[tokio::test]
async fn test_init_db_pool() {
    dotenv().ok();
    assert!(init_db_pool(true).await.is_ok())
}

#[tokio::test]
async fn test_db_connection() {
    dotenv().ok();
    let db_pool = init_db_pool(true).await.unwrap();
    let row: (i64,) = sqlx::query_as("SELECT 1")
        .fetch_one(&db_pool)
        .await
        .unwrap();

    assert_eq!(row.0, 1);
}
