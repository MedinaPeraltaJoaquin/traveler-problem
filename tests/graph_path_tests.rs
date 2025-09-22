#[cfg(test)]
mod tests {
    use traveler_problem::simulate_annealing::graph::Graph;
    use traveler_problem::simulate_annealing::path::Path;
    use traveler_problem::db::init_db;
    use traveler_problem::controllers::city_controller::get_cities_connection_by_ids_calculated;
    use traveler_problem::entity::city::City;
    use traveler_problem::entity::city_with_distance::CityWithDistance;
    use tokio::sync::OnceCell;
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;

    static DB: OnceCell<(Vec<CityWithDistance>, Vec<City>)> = OnceCell::const_new();

    async fn get_cities() -> &'static (Vec<CityWithDistance>, Vec<City>) {
        DB.get_or_init(|| async {
            let db_url = "sqlite:./tests/sql/tsp.bin";
            let sql_file = "./tests/sql/tsp_simulate_tests.sql";
            let db = init_db(sql_file, db_url).await.unwrap();
            let (cities_with_distance, cities) =
                get_cities_connection_by_ids_calculated(&db).await.unwrap();

            if cities_with_distance.is_empty() {
                panic!("No se encontraron conexiones");
            }

            (cities_with_distance, cities)
        }).await
    }

    #[tokio::test]
    async fn test_cities_loaded() {
        let (cities_with_distance, cities) = get_cities().await;
        assert!(!cities_with_distance.is_empty());
        assert!(!cities.is_empty());
    }

    #[tokio::test]
    async fn test_graph_creation() {
        let (cities_with_distance, cities) = get_cities().await;
        let mut graph = Graph::new(cities.clone(), cities_with_distance.clone());
        for city in cities {
            assert!(graph.get_node_by_position((city.id - 1) as usize).is_some());
        }

        for edge in cities_with_distance {
            let edge_data = &graph.get_edge(edge.origin, edge.city);
            assert!(edge_data.is_some());
            let (distance, calculated) = edge_data.unwrap();
            assert_eq!(distance, edge.distance_m);
            assert_eq!(calculated, 1);
        }

        let city_a = cities.first().unwrap();
        let city_b = cities.last().unwrap();
        let edge_data = &graph.get_edge(city_a.id, city_b.id);
        assert!(edge_data.is_some());
        let (distance, calculated) = edge_data.unwrap();
        assert!(distance > 0.0);
        assert_eq!(calculated, 0);
    }

    #[tokio::test]
    async fn test_path_constructor_cost(){
        let (cities_with_distance, cities) = get_cities().await;
        let mut graph = Graph::new(cities.clone(), cities_with_distance.clone());
        
        let city_ids = vec![1,2,3,4,74,163,164,165,166,167,169,326,327,328,329,330,489,490,
        491,492,493,494,495,653,654,655,658,666,814,815,816,817,818,819,978,979,980,981,1037,1073];
        let mut path = Path::new(city_ids.clone(), graph);
        let path_nodes = path.get_path();
        assert_eq!(*path_nodes, city_ids);

        let normalize = path.get_normalize();
        let mut diff = normalize - 173375434.909999967;
        assert!(diff.abs() < 0.1, "Expected normalize {}, got {} with diff {}", 173375434.909999967, normalize, diff);
        let distance_max = path.get_distance_max();
        diff = distance_max - 4940077.59;
        assert!(diff.abs() < 0.01, "Expected distance max {}, got {}with diff {}",4940077.59,distance_max,diff);
        let cost = path.get_cost();
        diff = (cost - 7598476.968976471).abs();
        assert!(diff < 0.01, "Expected cost {}, got {} with diff {}", 7598476.968976471, cost, diff);


        let city_ids = vec![1,2,3,4,5,6,7,8,9,11,12,14,16,17,19,20,22,23,25,26,27,74,75,151,163,
        164,165,166,167,168,169,171,172,173,174,176,179,181,182,183,184,185,186,187,213,297,326,327,328,
        329,330,331,332,333,334,336,339,340,343,344,345,346,347,349,350,351,352,353,444,483,489,490,491,
        492,493,494,495,496,499,500,501,502,504,505,507,508,509,510,511,512,520,652,653,654,655,656,657,
        658,660,661,662,663,665,666,667,668,670,671,673,674,675,676,678,815,816,817,818,819,820,821,822,
        823,825,826,828,829,832,837,839,840,978,979,980,981,982,984,985,986,988,990,991,995,999,1001,1003
        ,1004,1037,1038,1073,1075];

        graph = Graph::new(cities.clone(), cities_with_distance.clone());
        path = Path::new(city_ids.clone(), graph);
        let path_nodes = path.get_path();
        assert_eq!(*path_nodes, city_ids);

        let normalize = path.get_normalize();
        let mut diff = normalize - 723106584.20;
        assert!(diff.abs() < 0.1, "Expected normalize {}, got {} with diff {}", 723106584.200000286, normalize, diff);
        let cost = path.get_cost();
        diff = (cost - 6161590.480045998).abs();
        assert!(diff < 0.01, "Expected cost {}, got {} with diff {}", 6161590.480045998, cost, diff);
        let distance_max = path.get_distance_max();
        diff = distance_max - 4978506.48;
        assert!(diff.abs() < 0.01, "Expected distance max {}, got {}with diff {}",4940077.59,distance_max,diff); 
    }

    #[tokio::test]
    async fn test_path_get_vecino(){
        let (cities_with_distance, cities) = get_cities().await;
        let graph = Graph::new(cities.clone(), cities_with_distance.clone());
        
        let mut city_ids = vec![1,2,3,4,74,163,164,165,166,167,169,326,327,328,329,330,489,490,
        491,492,493,494,495,653,654,655,658,666,814,815,816,817,818,819,978,979,980,981,1037,1073];
        let mut path = Path::new(city_ids.clone(), graph);
        let seed = 1 as u64;
        let mut random_1 = StdRng::seed_from_u64(seed);
        let mut random_2 = StdRng::seed_from_u64(seed);
        let city_ids_length = city_ids.len();
        for _ in 0..city_ids_length {
            let expect_index1 = random_1.gen_range(1..city_ids_length) as usize;
            let expect_index2 = random_1.gen_range(0..expect_index1) as usize;
            if expect_index1 == expect_index2 {
                continue;
            }
            let result = path.get_vecino(&mut random_2);
            assert_eq!(result.0,expect_index1);
            assert_eq!(result.1,expect_index2);
            let expect_apply = path.apply_vecino();
            assert!(expect_apply);
            city_ids.swap(expect_index1, expect_index2);
            assert_eq!(*path.get_path(), city_ids)
        }

        for _ in 0..city_ids_length {
            let actual_cost = path.get_cost();
            let vecino = path.get_min(&mut random_1,actual_cost,1000);
            if vecino.2 != 0.0 {
                assert!(vecino.2 < actual_cost);   
            }
        }
    }
}
