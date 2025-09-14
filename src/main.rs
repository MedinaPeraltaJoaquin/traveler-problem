mod simulate_annealing;
mod config;
mod db;
mod entity;
mod controllers;

use config::Config;
use simulate_annealing::simulate_annealing::SimulatedAnnealing;
use simulate_annealing::graph::Graph;
use simulate_annealing::path::Path;
use simulate_annealing::initial_temperature::InitialTemperature;
use controllers::city_controller::get_cities_connection_by_ids_calculated;
use db::init_db;
use core::panic;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();

    //Aqui deberia de leer la entrada
    let city_ids = vec![1,2,3,4,5,6,7,8,9,11,12,14,16,17,19,20,22,23,25,26,27,74,75,151,163,
        164,165,166,167,168,169,171,172,173,174,176,179,181,182,183,184,185,186,187,213,297,326,327,328,
        329,330,331,332,333,334,336,339,340,343,344,345,346,347,349,350,351,352,353,444,483,489,490,491,
        492,493,494,495,496,499,500,501,502,504,505,507,508,509,510,511,512,520,652,653,654,655,656,657,
        658,660,661,662,663,665,666,667,668,670,671,673,674,675,676,678,815,816,817,818,819,820,821,822,
        823,825,826,828,829,832,837,839,840,978,979,980,981,982,984,985,986,988,990,991,995,999,1001,1003
        ,1004,1037,1038,1073,1075];

    let db = init_db(&config.sql_path, &config.db_url).await?;

    let (cities_with_distance, cities) =
        get_cities_connection_by_ids_calculated(&db).await?;

    if cities_with_distance.is_empty() {
        panic!("No se encontraron conexiones")
    }

    let graph = Graph::new(cities, cities_with_distance);
    let path = Path::new(city_ids.clone(), graph);
 
    let mut initial_temperature = InitialTemperature::new(
        config.initial_temperature, 
        config.percentage, 
        config.e_p, 
        path.clone(), 
        config.n, 
        config.seed
    );

    let temperature = initial_temperature.get_initial_t(config.limit * config.size_lote);

    let mut simulated_annealing = SimulatedAnnealing::new(
        temperature,
        config.cooling_rate,
        path,
        config.size_lote,
        config.seed,
        config.e_s,
        config.limit
    );

    let total = simulated_annealing.accept_threshold();
    let mut best_solution = simulated_annealing.get_best_solution().clone();
    println!("Mejor solucion encontrada: {:?}", best_solution.get_path());
    println!("Costo de la mejor solucion: {}", best_solution.get_cost());
    println!("Número de iteracciónes: {}",total);
    println!("Semilla: {}",config.seed as i32);
    Ok(())
}
