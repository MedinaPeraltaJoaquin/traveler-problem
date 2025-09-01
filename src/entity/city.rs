use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct City {
    pub id : i32,
    pub latitude: f64,
    pub longitude: f64
}

impl City {
    pub fn new(id: i32, latitude: f64, longitude: f64) -> Self {
        City { id, latitude, longitude }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn set_latitude(&mut self, latitude: f64) {
        self.latitude = latitude;
    }

    pub fn set_longitude(&mut self, longitude: f64) {
        self.longitude = longitude;
    }

    pub fn distance(&self, other: &City) -> f64 {
        let radius_earth_m: f64 = 6373000.0;
        
        let delta_lat_rad: f64 = other.latitude.to_radians() - self.latitude.to_radians();
        let delta_lon_rad: f64 = other.longitude.to_radians() - self.longitude.to_radians();

        let a: f64 = (delta_lat_rad / 2.0).sin().powi(2)
            + (self.latitude.to_radians()).cos() * (other.latitude.to_radians()).cos() * (delta_lon_rad / 2.0).sin().powi(2);
        let c: f64 = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        radius_earth_m * c
    }

    pub fn to_string(&self) -> String {
        format!("City {{ id: {}, latitude: {}, longitude: {} }}", self.id, self.latitude, self.longitude)
    }
}