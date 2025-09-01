use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CityWithDistance {
    pub city: i32,
    pub origin: i32,
    pub distance_m: f64,
}

impl CityWithDistance {
    pub fn new(city: i32, origin: i32, distance_m: f64) -> Self {
        CityWithDistance { city, origin, distance_m }
    }

    pub fn get_city(&self) -> i32 {
        self.city
    }

    pub fn get_origin(&self) -> i32 {
        self.origin
    }

    pub fn get_distance_m(&self) -> f64 {
        self.distance_m
    }

    pub fn set_city(&mut self, city: i32) {
        self.city = city;
    }

    pub fn set_origin(&mut self, origin: i32) {
        self.origin = origin;
    }

    pub fn set_distance_m(&mut self, distance_m: f64) {
        self.distance_m = distance_m;
    }
}
