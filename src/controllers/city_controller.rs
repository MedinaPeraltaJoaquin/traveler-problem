use crate::db::DbPool;
use crate::entity::city::City;
use crate::entity::city_with_distance::CityWithDistance;
use sqlx::Error;

async fn fetch_connected_cities(pool: &DbPool, city_id: i32) -> Result<Vec<City>,Error> {
    let connected_ids: Vec<i32> = sqlx::query_scalar(
        r#"
        SELECT id_city_2 FROM connections WHERE id_city_1 = ?
        UNION
        SELECT id_city_1 FROM connections WHERE id_city_2 = ?
        "#
    )
    .bind(city_id)
    .bind(city_id)
    .fetch_all(pool)
    .await?;

    if connected_ids.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = connected_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!("SELECT id, latitude, longitude FROM cities WHERE id IN ({})", placeholders);

    let mut q = sqlx::query_as::<_, City>(&query);
    for id in connected_ids {
        q = q.bind(id);
    }

    let cities: Vec<City> = q.fetch_all(pool).await?;
    Ok(cities)
}

pub async fn get_cities_connection_by_id(pool: &DbPool, city_id: i32) -> Result<Vec<City>, Error> {
    fetch_connected_cities(pool, city_id).await
}

pub async fn get_cities_connection_by_id_calculated(pool: &DbPool, city_id: i32) -> Result<Vec<CityWithDistance>, Error> {
    let origin: City = sqlx::query_as("SELECT id, latitude, longitude FROM cities WHERE id = ?")
    .bind(city_id)
    .fetch_one(pool)
    .await?;

    if origin.id == 0 {
        return Ok(vec![]);
    }

    let connected_cities = fetch_connected_cities(pool, city_id).await?;

    let result: Vec<CityWithDistance> = connected_cities
        .into_iter()
        .map(|c: City| CityWithDistance {
            distance_m: origin.distance(&c),
            city: c.id,
            origin: origin.id,
        })
        .collect();

    Ok(result)
}

