#[cfg(test)]
mod tests {
    use traveler_problem::utils::write_report::WriteReport;
    use traveler_problem::simulate_annealing::graph::Graph;
    use traveler_problem::simulate_annealing::path::Path;
    use traveler_problem::db::init_db;
    use traveler_problem::controllers::city_controller::get_cities_connection_by_ids_calculated;
    use traveler_problem::entity::city::City;
    use traveler_problem::entity::city_with_distance::CityWithDistance;
    use tokio::sync::OnceCell;
    use tempfile::tempdir;
    use std::time::Duration;

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

    async fn create_report(verbose: bool) -> (WriteReport,Path) {
        let (cities_with_distance, cities) = get_cities().await;
        let graph = Graph::new(cities.clone(), cities_with_distance.clone());
        
        let city_ids = vec![1,2,3,4,74,163,164,165,166,167,169,326,327,328,329,330,
                            489,490,491,492,493,494,495,653,654,655,658,666,
                            814,815,816,817,818,819,978,979,980,981,1037,1073];
        let path = Path::new(city_ids.clone(), graph);

        (WriteReport::new(
            path.clone(),
            5,
            42,
            Duration::from_secs(2),
            verbose
        ),path)
    }

    #[tokio::test]
    async fn test_generate_report() {
        let (mut report,mut path) = create_report(true).await;

        let content = report.generate_report();
        let path_str = format!("{:?}", path.get_path());

        assert!(content.contains(&format!("Mejor solucion encontrada: {}", path_str)));
        assert!(content.contains(&format!("Costo de la mejor solucion: {}", path.get_cost())));
        assert!(content.contains("Número de iteracciónes: 5"));
        assert!(content.contains("Semilla: 42"));
        assert!(content.contains("Tiempo transcurrido:"));
    }

    #[tokio::test]
    async fn test_print_report_verbose() {
        let (mut report,mut path) = create_report(true).await;

        let mut buffer: Vec<u8> = Vec::new();

        report.print_report_to(&mut buffer).unwrap();
        let output_str = String::from_utf8(buffer).unwrap();
        let path_str = format!("{:?}", path.get_path());
        assert!(output_str.contains(&format!("Mejor solucion encontrada: {}", path_str)));
        assert!(output_str.contains(&format!("Costo de la mejor solucion: {}", path.get_cost())));
        assert!(output_str.contains("Número de iteracciónes: 5"));
        assert!(output_str.contains("Semilla: 42"));
        assert!(output_str.contains("Tiempo transcurrido:"));
    }

    #[tokio::test]
    async fn test_print_report_without_verbose() {
        let (mut report,mut path) = create_report(false).await;

        let mut buffer: Vec<u8> = Vec::new();

        report.print_report_to(&mut buffer).unwrap();
        let output_str = String::from_utf8(buffer).unwrap();
        assert!(output_str.contains(&format!("Mejor costo: {}, iteraciones: 5, tiempo:", path.get_cost())));
    }

    #[tokio::test]
    async fn test_save_to_file_creates_file() {
        let (mut report, path) = create_report(true).await;

        let path_str = format!("{:?}", path.get_path());
        let dir = tempdir().unwrap();
        let (file_path, _) = report.save_to_file(dir.path().to_str().unwrap()).unwrap();

        assert!(std::path::Path::new(&file_path).exists());
        let content = std::fs::read_to_string(file_path).unwrap();
        assert!(content.contains(&format!("Mejor solucion encontrada: {}", path_str)));
    }

    #[tokio::test]
    async fn test_multiple_reports_unique_files() {
        let (mut report1,_) = create_report(true).await;
        let (mut report2,_) = create_report(true).await;

        let dir = tempdir().unwrap();
        let (file1, _) = report1.save_to_file(dir.path().to_str().unwrap()).unwrap();
        let (file2, _) = report2.save_to_file(dir.path().to_str().unwrap()).unwrap();

        assert_ne!(file1, file2);
        assert!(std::path::Path::new(&file1).exists());
        assert!(std::path::Path::new(&file2).exists());
    }
}
