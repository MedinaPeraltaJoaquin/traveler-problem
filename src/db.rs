use sqlx::{Pool, Sqlite, SqlitePool};
use tokio::fs;

pub type DbPool = Pool<Sqlite>;

async fn check_tables(pool: &DbPool) -> Result<bool, sqlx::Error> {
    let row: (i32,) = sqlx::query_as(
        r#"
        SELECT COUNT(name) as count FROM sqlite_master 
        WHERE type='table' AND name IN ('cities', 'connections');
        "#
    )
    .fetch_one(pool)
    .await?;
    Ok(row.0 == 2)
}

pub async fn init_db(sql_path: &str, db_url: &str) -> Result<DbPool, sqlx::Error> {
    let pool: Pool<Sqlite> = SqlitePool::connect(db_url).await?;
    if check_tables(&pool).await? {
        return Ok(pool);
    }
    println!("Initializing database from {}", sql_path);
    let sql: String = fs::read_to_string(sql_path).await.expect("Failed to read SQL file");
    sqlx::query(&sql).execute(&pool).await?;
    Ok(pool)
}