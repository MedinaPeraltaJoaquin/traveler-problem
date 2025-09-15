use sqlx::FromRow;

#[derive(Debug, FromRow,Clone)]
pub struct City {
    pub id : i32,
    pub latitude: f64,
    pub longitude: f64
}

impl City {

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn distance(&self, other: &City) -> f64 {
        let radius_earth_m: f64 = 6_373_000.0;

        let lat1 = self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let long1 = self.longitude.to_radians();
        let long2 = other.longitude.to_radians(); 
        let delta_lat = lat2 - lat1;
        let delta_lon = long2 - long1;

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);

        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        radius_earth_m * c
    }
}