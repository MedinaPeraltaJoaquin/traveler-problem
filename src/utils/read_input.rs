use std::fmt;
use std::fs;

#[derive(Debug)]
pub enum InputError {
    NoArgs,
    InvalidSeed,
    InvalidPath,
    MissingPathValue,
    InvalidArgumentSeed,
    MissingArgument,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::NoArgs => write!(f, "No se pasaron argumentos"),
            InputError::InvalidSeed => write!(f, "Seed inválido"),
            InputError::InvalidPath => write!(f, "Ruta inválida"),
            InputError::MissingPathValue => write!(f, "No hay camino válido"),
            InputError::InvalidArgumentSeed => write!(f, "No se pueden usar ambos argumentos"),
            InputError::MissingArgument => write!(f,"Faltan argumentos"),
        }
    }
}

impl std::error::Error for InputError {}


pub struct ReadInput {
    pub args : Vec<String>,
    pub seeds : Vec<i32>,
    pub path : Vec<i32>,
}

impl ReadInput {
    pub fn new(args : Vec<String>) -> Result<Self,InputError>{
        if args.len() == 1 {
            return Err(InputError::NoArgs);
        }

        Ok(ReadInput {
            args : args,
            seeds : vec![],
            path : vec![]
        })
    }

    pub fn get_path(&mut self) -> Result<Vec<i32>,InputError> {
        if !self.path.is_empty() {
            return Ok(self.path.clone())
        }

        let position = if let Some(pos) = self.get_position_flag("-p") {
            if pos + 1 < self.args.len() {
                pos
            } else {
                return Err(InputError::InvalidPath);
            }
        } else {
            return Err(InputError::InvalidPath);
        };

        let next_arg = &self.args[position + 1];

        let parsed_path = if next_arg.ends_with(".tsp") {
            let content = fs::read_to_string(next_arg)
                .map_err(|_| InputError::InvalidPath)?;

            let mut parsed_path = Vec::new();

            for (line_no, line) in content.lines().enumerate() {
                for token in line.split_whitespace() {
                    match token.parse::<i32>() {
                        Ok(val) => parsed_path.push(val),
                        Err(_) => {
                            eprintln!("Error al parsear token '{}' en línea {}", token, line_no + 1);
                            return Err(InputError::InvalidPath);
                        }
                    }
                }
            }

            parsed_path
        } else {
            let mut path_vec = Vec::new();
            for arg in &self.args[position + 1..] {
                if arg.starts_with('-') {
                    break;
                }
                match arg.parse::<i32>() {
                    Ok(val) => path_vec.push(val),
                    Err(_) => return Err(InputError::InvalidPath),
                }
            }
            if path_vec.is_empty() {
                return Err(InputError::MissingPathValue);
            }
            path_vec
        };

        self.path = parsed_path;
        Ok(self.path.clone())
    }

    pub fn get_verbose(&self) -> bool {
        self.get_flag("-v")
    }

    pub fn get_svg(&self) -> bool {
        self.get_flag("-svg")
    }
    
    pub fn get_help(&self) -> bool {
        self.get_flag("-h") || self.get_flag("--help")
    }

    pub fn get_recover(&self) -> bool {
        self.get_flag("--recover")
    }

    pub fn get_seed(&mut self) -> Result<Vec<i32>, InputError> {
        if !self.seeds.is_empty() {
            return Ok(self.seeds.clone());
        }

        let pos_s = self.get_position_flag("-s");
        let pos_rs = self.get_position_flag("-rs");

        match (pos_s, pos_rs) {
            (Some(_), Some(_)) => {
                return Err(InputError::InvalidArgumentSeed);
            }
            (Some(pos), None) => {
                if self.args.len() < pos + 1 {
                    return Err(InputError::MissingArgument);
                }

                let start: i32 = self.args[pos + 1].parse()
                    .map_err(|_| InputError::InvalidSeed)?;

                if self.args.len() <= pos + 2 || self.args[pos + 2].starts_with('-') {
                    let seeds = vec![start];
                    self.seeds = seeds.clone();
                    return Ok(seeds);
                }

                let end: i32 = self.args[pos + 2].parse()
                    .map_err(|_| InputError::InvalidSeed)?;

                if start > end {
                    return Err(InputError::InvalidSeed);
                }

                let seeds: Vec<i32> = (start..=end).collect();
                self.seeds = seeds.clone();
                return Ok(seeds);
            }
            (None, Some(pos)) => {
                if self.args.len() <= pos + 1 {
                    return Err(InputError::MissingArgument);
                }

                let n: usize = self.args[pos + 1].parse()
                    .map_err(|_| InputError::InvalidSeed)?;

                if n == 0 {
                    return Err(InputError::InvalidSeed);
                }

                use rand::Rng;
                let mut rng = rand::thread_rng();
                let seeds: Vec<i32> = (0..n).map(|_| rng.r#gen()).collect();
                self.seeds = seeds.clone();
                return Ok(seeds);
            }
            (None, None) => {
                Err(InputError::InvalidSeed)
            }
        }
    }

    pub fn print_help(&self) {
        println!("Uso: programa [opciones]");
        println!();
        println!("Opciones:");
        println!("  -h, --help         Muestra esta ayuda y termina");
        println!("  -v                 Activa el modo verbose");
        println!("  -p <path>          Ruta explícita como lista de enteros o archivo .tsp");
        println!("  -s <I> <F>         Genera semillas en el rango [I, F]");
        println!("  -s <n>             Inicializa con la semilla n");
        println!("  -rs <n>            Genera n semillas aleatorias");
        println!("  -svg               Activa el modo de salida SVG");
    }



    fn get_flag(&self, flag : &'static str) -> bool {
        self.args.iter().any(|arg| arg == flag)
    }

    fn get_position_flag(&self, flag: &str) -> Option<usize> {
        self.args.iter().position(|arg| arg == flag)
    }
}