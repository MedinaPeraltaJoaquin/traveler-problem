use crate::simulate_annealing::path::Path;
use std::fs::{File, create_dir_all};
use std::time::Duration;
use chrono::Local;
use std::io::{self, Write};

pub struct WriteReport {
    verbose_mode: bool,
    report: String,
    path: Path,
    total: usize,
    seed: i32,
    duration: Duration,
}

impl WriteReport {
    pub fn new(path: Path, total: usize, seed: i32, duration: Duration, verbose_mode: bool) -> Self {
        WriteReport {
            verbose_mode,
            report: String::new(),
            path,
            total,
            seed,
            duration,
        }
    }

    pub fn generate_report(&mut self) -> &str {
        self.report = format!(
            "Mejor solucion encontrada: {:?}\n\
            Costo de la mejor solucion: {}\n\
            Número de iteracciónes: {}\n\
            Semilla: {}\n\
            Tiempo transcurrido: {:?}\n",
            self.path.get_path().clone(),
            self.path.get_cost(),
            self.total,
            self.seed,
            self.duration
        );
        &self.report
    }

    pub fn print_report_to<W: Write>(&mut self, mut writer: W) -> io::Result<()> {
            let content = if self.verbose_mode {
                self.generate_report()
            } else {
                &format!(
                    "Mejor costo: {}, iteraciones: {}, tiempo: {:?}",
                    self.path.get_cost(),
                    self.total,
                    self.duration
                )
            };

            writeln!(writer, "{}", content)
        }

    pub fn print_report(&mut self) {
        let _ = self.print_report_to(io::stdout());
    }

    pub fn save_to_file(&mut self, dir_path: &str) -> Result<(String,String), std::io::Error> {
        create_dir_all(dir_path)?;
        let seed = self.seed;
        let report_content = self.generate_report();

        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d_%H-%M-%S-%3f").to_string();

        let file_name = format!("reporte_{}_{}",seed, timestamp);
        let file_path = format!("{}/{}.txt", dir_path, file_name);

        let mut file = File::create(&file_path)?;
        file.write_all(report_content.as_bytes())?;

        Ok((file_path,file_name))
    }
}
