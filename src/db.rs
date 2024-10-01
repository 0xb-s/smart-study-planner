use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::env;


pub async fn establish_connection() -> SqlitePool {
    dotenv().ok(); // create your own .env here

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool")
}
