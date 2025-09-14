
#[derive(Debug,Clone)]
pub struct CityWithDistance {
    pub city: i32,
    pub origin: i32,
    pub distance_m: f64,
}

impl CityWithDistance {

    pub fn get_city(&self) -> i32 {
        self.city
    }

    pub fn get_origin(&self) -> i32 {
        self.origin
    }
}
