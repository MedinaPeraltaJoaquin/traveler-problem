use traveler_problem::db;
use traveler_problem::entity::city::City;
use traveler_problem::entity::city_with_distance::CityWithDistance;
use traveler_problem::controllers::city_controller::{get_cities_connection_by_id,get_cities_connection_by_id_calculated};
use sqlx::{Pool, Sqlite};

async fn setup_test_db() -> Pool<Sqlite> {
    let db_url = "sqlite::memory:?cache=shared";
    let sql_file: &'static str = "./tests/sql/tsp.sql";
    let pool: Pool<Sqlite> = db::init_db(sql_file, &db_url).await.unwrap();
    pool
}

#[tokio::test]
async fn test_get_cities_connection_by_id() {
    let pool: Pool<Sqlite> = setup_test_db().await;
    let result: Vec<City> = get_cities_connection_by_id(&pool, 1).await.unwrap();

    assert_eq!(result.len(), 3);
    let ids: Vec<i32> = result.iter().map(|c: &City| c.id).collect();
    assert!(ids.contains(&7));
    assert!(ids.contains(&9));
    assert!(ids.contains(&19));
}

#[tokio::test]
async fn test_get_cities_connection_by_id_calculated() {
    let pool: Pool<Sqlite> = setup_test_db().await;
    let result: Vec<CityWithDistance> = get_cities_connection_by_id_calculated(&pool, 1).await.unwrap();

    assert_eq!(result.len(), 3);

    let ids: Vec<i32> = result.iter().map(|c| c.city).collect();
    assert!(ids.contains(&7));
    assert!(ids.contains(&9));
    assert!(ids.contains(&19));

    for c in result {
        assert!(c.distance_m > 0.0);
    }
}
