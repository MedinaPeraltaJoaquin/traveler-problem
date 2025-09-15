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
        return Err(Error::RowNotFound);
    }

    let placeholders = connected_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!("SELECT id, latitude, longitude FROM cities WHERE id IN ({})", placeholders);

    let mut q = sqlx::query_as::<_, City>(&query);
    for id in connected_ids {
        q = q.bind(id);
    }

    let cities: Vec<City> = q.fetch_all(pool).await?;
    if cities.is_empty() {
        return Err(Error::RowNotFound);
    }
    Ok(cities)
}

pub async fn get_cities_connection_by_id_calculated(pool: &DbPool, city : City) -> Result<Vec<CityWithDistance>, Error> {
    let connected_cities = fetch_connected_cities(pool, city.get_id()).await?;

    let result: Vec<CityWithDistance> = connected_cities
        .into_iter()
        .map(|c: City| CityWithDistance {
            distance_m: city.distance(&c),
            city: c.get_id(),
            origin: city.get_id(),
        })
        .collect();

    Ok(result)
}

pub async fn get_cities_connection_by_ids_calculated(pool: &DbPool) -> Result<(Vec<CityWithDistance>, Vec<City>), Error> {
    let mut result: Vec<CityWithDistance> = Vec::new();
    let cities: Vec<City> = sqlx::query_as("SELECT id, latitude, longitude FROM cities ORDER BY id ASC")
        .fetch_all(pool)
        .await?;

    if cities.is_empty() {
        return Err(Error::RowNotFound);
    }

    for city in &cities {
        let mut connections = get_cities_connection_by_id_calculated(pool, city.clone()).await?;
        result.append(&mut connections);
    }
    
    Ok((result, cities))
}
