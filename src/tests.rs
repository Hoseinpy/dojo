mod helpers_tests {
    use dotenvy::dotenv;

    use crate::helpers::{get_timestamp, get_version, init_db_pool, print_help, readable_datetime};

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
        print_help();
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
}

mod models_tests {
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
