use sqlx::{Pool, Sqlite, SqlitePool};
use tokio::fs;

pub type DbPool = Pool<Sqlite>;

pub async fn init_db(sql_path: &str, db_url: &str) -> Result<DbPool, sqlx::Error> {
    let pool: Pool<Sqlite> = SqlitePool::connect(db_url).await?;
    let sql: String = fs::read_to_string(sql_path).await.expect("Failed to read SQL file");
    sqlx::query(&sql).execute(&pool).await?;
    Ok(pool)
}