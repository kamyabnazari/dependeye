use sqlx::SqlitePool;

pub async fn connect_db() -> Result<SqlitePool, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    SqlitePool::connect(&db_url).await
}

pub async fn check_connection() -> bool {
    match connect_db().await {
        Ok(_) => true,
        Err(e) => {
            eprintln!("DB connection error: {}", e);
            false
        }
    }
}
