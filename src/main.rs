mod simulate_annealing;
mod config;
mod db;
mod entity;
mod controllers;
mod utils;

use utils::read_input::ReadInput;
use config::Config;
use utils::write_report::WriteReport;
use utils::generate_svg::GenerateSvg;
use simulate_annealing::simulate_annealing::SimulatedAnnealing;
use simulate_annealing::graph::Graph;
use simulate_annealing::path::Path;
use simulate_annealing::initial_temperature::InitialTemperature;
use controllers::city_controller::get_cities_connection_by_ids_calculated;
use db::init_db;
use core::panic;
use std::env;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut read_input = match ReadInput::new(args) {
        Ok(read) => read,
        Err(e) => {
            panic!("Error al leer argumentos: {:?}\nUtilice --help o -h", e);
        }
    };

    if read_input.get_help() {
        read_input.print_help();
        return Ok(());
    }

    let verbose_mode = read_input.get_verbose();
    let svg_mode = read_input.get_svg();

    let seeds = match read_input.get_seed() {
        Ok(seeds) => seeds,
        Err(e) => {
            panic!("Error al leer la semilla: {:?}", e);
        }
    };

    let city_ids = match read_input.get_path() {
        Ok(path) => path,
        Err(e) => {
            panic!("Error al leer el path: {:?}", e);
        }
    };

    let config = Config::from_env();
    let db = init_db(&config.sql_path, &config.db_url).await?;

    let (cities_with_distance, cities) =
        get_cities_connection_by_ids_calculated(&db).await?;

    if cities_with_distance.is_empty() {
        panic!("No se encontraron conexiones")
    }

    let graph = Graph::new(cities, cities_with_distance);
    let path = Path::new(city_ids.clone(), graph);
 
    let start = Instant::now();
    for seed in &seeds {
        let start_process = Instant::now();
        let mut initial_temperature = InitialTemperature::new(
            config.initial_temperature, 
            config.percentage, 
            config.e_p, 
            path.clone(), 
            config.n, 
            seed.clone() as u64
        );

        let temperature = initial_temperature.get_initial_t(config.limit * config.size_lote);
        let mut simulated_annealing = SimulatedAnnealing::new(
            temperature,
            config.cooling_rate,
            path.clone(),
            config.size_lote,
            seed.clone() as u64,
            config.e_s,
            config.limit
        );

        let mut generate_svg = GenerateSvg::new(
            svg_mode,
            2000,
            1000,
            50
        );
        let total = simulated_annealing.accept_threshold(&mut generate_svg);
        let best_solution = simulated_annealing.get_best_solution().clone();
        let mut report = WriteReport::new(
            best_solution,
            total,
            seed.clone(),
            start_process.elapsed(),
            verbose_mode
        );

        report.print_report();
        let (file_path,file_name) = match report.save_to_file("./reports") {
            Ok(file) => file,
            Err(er) => {
                panic!("Error al guardar el archivo: {:?}", er);
            }
        };

        println!("Reporte guardado en: {}", file_path);
        if svg_mode {
            if let Err(er) = generate_svg.save_svg("./svg", file_name) {
                panic!("Error al guardar el archivo: {:?}", er);
            }
        }
        
    }

    let duration = start.elapsed();
    println!("Tiempo para procesar {} semillas: {:?}",seeds.len(),duration);
    Ok(())
}
