use traveler_problem::entity::city_with_distance;

#[test]
pub fn test_city_with_distance_constructor() {
    let city_distance = city_with_distance::CityWithDistance { city: 1, origin: 2, distance_m: 1000.0 };
    assert_eq!(city_distance.get_city(), 1);
    assert_eq!(city_distance.get_origin(), 2);
}
