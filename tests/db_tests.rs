use traveler_problem::db;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_init_db_creates_tables() {
    let temp_db: NamedTempFile = NamedTempFile::new().unwrap();
    let db_url: String = format!("sqlite:{}", temp_db.path().to_str().unwrap());

    let sql_file: &'static str = "./tests/sql/tsp.sql";

    let pool: sqlx::Pool<sqlx::Sqlite> = db::init_db(sql_file, &db_url).await.unwrap();

    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='cities'")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(row.0, 1);

    let row2: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='connections'")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(row2.0, 1);
}


