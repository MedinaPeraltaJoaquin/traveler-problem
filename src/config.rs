use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub initial_temperature: f64,
    pub cooling_rate: f64,
    pub size_lote: usize,
    pub db_url: String,
    pub sql_path: String,
    pub e_s : f64,
    pub limit: usize,
    pub e_p : f64,
    pub percentage : f64,
    pub n : usize
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let initial_temperature = env::var("INITIAL_TEMPERATURE")
            .expect("Falta INITIAL_TEMPERATURE en .env")
            .parse::<f64>()
            .expect("INITIAL_TEMPERATURE debe ser un número");

        let cooling_rate = env::var("COOLING_RATE")
            .expect("Falta COOLING_RATE en .env")
            .parse::<f64>()
            .expect("COOLING_RATE debe ser un número");

        let size_lote = env::var("SIZE_LOTE")
            .expect("Falta SIZE_LOTE en .env")
            .parse::<usize>()
            .expect("SIZE_LOTE debe ser un número entero");

        let e_s = env::var("E_S")
            .expect("Falta E_S en .env")
            .parse::<f64>()
            .expect("E_S debe de ser un núumero f64");

        let limit = env::var("LIMIT")
            .expect("Falta LIMIT en .env")
            .parse::<usize>()
            .expect("LIMIT debe ser un número entero");

        let n = env::var("N")
            .expect("Falta N en .env")
            .parse::<usize>()
            .expect("N debe ser un número entero");

        let e_p = env::var("E_P")
            .expect("Falta E_P en .env")
            .parse::<f64>()
            .expect("E_P debe de ser un núumero f64");

        let percentage = env::var("PERCENTAGE")
            .expect("Falta PERCENTAGE en .env")
            .parse::<f64>()
            .expect("PERCENTAGE debe de ser un núumero f64");    
        
        let db_url = env::var("DATABASE_URL").expect("Falta DATABASE_URL en .env");
        let sql_path = env::var("SQL_PATH").expect("Falta SQL_PATH en .env");

        Config {
            initial_temperature,
            cooling_rate,
            size_lote,
            db_url,
            sql_path,
            e_s,
            limit,
            e_p,
            percentage,
            n
        }
    }
}
