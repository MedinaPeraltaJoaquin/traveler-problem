use traveler_problem::entity::city_with_distance;

#[test]
pub fn test_city_with_distance_constructor() {
    let city_distance = city_with_distance::CityWithDistance::new(1, 2, 1000.0);
    assert_eq!(city_distance.get_city(), 1);
    assert_eq!(city_distance.get_origin(), 2);
    assert_eq!(city_distance.get_distance_m(), 1000.0);
}

#[test]
pub fn test_city_with_distance_setters() {
    let mut city_distance = city_with_distance::CityWithDistance::new(1, 2, 1000.0);
    city_distance.set_city(3);
    city_distance.set_origin(4);
    city_distance.set_distance_m(2000.0);
    assert_eq!(city_distance.get_city(), 3);
    assert_eq!(city_distance.get_origin(), 4);
    assert_eq!(city_distance.get_distance_m(), 2000.0);
}
