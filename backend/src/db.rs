use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn connect_db(db_url: &str) -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Could not connect to database")
}