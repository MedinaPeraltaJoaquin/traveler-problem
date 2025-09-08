use traveler_problem::db;
use traveler_problem::entity::city::City;
use traveler_problem::entity::city_with_distance::CityWithDistance;
use traveler_problem::controllers::city_controller::{get_cities_connection_by_id_calculated};
use sqlx::{Pool, Sqlite};

async fn setup_test_db() -> Pool<Sqlite> {
    let db_url = "sqlite::memory:?cache=shared";
    let sql_file: &'static str = "./tests/sql/tsp.sql";
    let pool: Pool<Sqlite> = db::init_db(sql_file, &db_url).await.unwrap();
    pool
}


#[tokio::test]
async fn test_get_cities_connection_by_id_calculated() {
    let pool: Pool<Sqlite> = setup_test_db().await;
    let city = City { id: 1, latitude: 40.7128, longitude: -74.0060 };
    let result: (Vec<CityWithDistance>, City) = get_cities_connection_by_id_calculated(&pool, &city).await.unwrap();

    assert_eq!(result.1.get_id(), 1);
    assert_eq!(result.0.len(), 3);

    let ids: Vec<i32> = result.0.iter().map(|c| c.city.0).collect();
    assert!(ids.contains(&7));
    assert!(ids.contains(&9));
    assert!(ids.contains(&19));

    for c in result.0 {
        assert!(c.distance_m > 0.0);
    }
}

