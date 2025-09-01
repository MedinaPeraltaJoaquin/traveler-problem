use traveler_problem::entity::city;

#[test]
pub fn test_city_constructor() {
    let city = city::City::new(1, 52.5200, 13.4050);
    assert_eq!(city.get_id(), 1);
    assert_eq!(city.get_latitude(), 52.5200);
    assert_eq!(city.get_longitude(), 13.4050);
}

#[test]
pub fn test_city_setters() {
    let mut city = city::City::new(1, 52.5200, 13.4050);
    city.set_id(2);
    city.set_latitude(48.8566);
    city.set_longitude(2.3522);
    assert_eq!(city.get_id(), 2);
    assert_eq!(city.get_latitude(), 48.8566);
    assert_eq!(city.get_longitude(), 2.3522);
}

#[test]
pub fn test_city_to_string() {
    let city = city::City::new(1, 52.5200, 13.4050);
    let city_str = city.to_string();
    assert_eq!(city_str, "City { id: 1, latitude: 52.52, longitude: 13.405 }");
}

#[test]
pub fn test_city_distance(){
    let city_tests : Vec<(i32,f64,f64)> = vec![
        (1,35.68500000000000227,139.7510000000000047),
        (2,31.0456000000000003,121.4000000000000056),
        (3,18.97500000000000143,72.82580000000000097),
        (7,14.60420000000000051,120.9819999999999994),
        (9,37.59850000000000136,126.9779999999999945),
        (19,39.92889999999999873,116.3880000000000052),
        (14,-6.174439999999999707,106.8289999999999935),
        (11,41.01859999999999929,28.96470000000000055)
    ];

    let distance_test : Vec<(i32,i32,f64)> = vec![
        (1,7,2999396.229999999982),
        (1,9,1158707.310000000055),
        (1,19,2100171.729999999982),
        (2,7,1829270.909999999917),
        (2,9,890547.0200000000186),
        (2,14,4420586.230000000448),
        (2,19,1086867.429999999934),
        (3,11,4815550.849999999628)
    ];

    let city_origin = city::City::new(1, 35.6850, 139.7510);
    for (id,lat,lon) in city_tests {
        let city = city::City::new(id, lat, lon);
        let dist = city_origin.distance(&city);
        for (o,d,t) in &distance_test {
            if *o == city_origin.get_id() && *d == city.get_id() {
                let diff = (dist - t).abs();
                assert!(diff < 0.01, "Distance between city {} and city {} expected {}, got {}, diff {}", o, d, t, dist, diff);
            }
        }
    }
}