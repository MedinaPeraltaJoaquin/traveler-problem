use std::cmp::Ordering;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;


#[derive(Debug, Clone)]
struct Reporte {
    semilla: i32,
    tiempo: f64,
    costo: f64
}

fn procesar_archivo(path: &Path) -> io::Result<Option<Reporte>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut semilla: Option<i32> = None;
    let mut tiempo: Option<f64> = None;
    let mut costo: Option<f64> = None;

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("Semilla:") {
            if let Some(val) = line.split(':').nth(1) {
                semilla = val.trim().parse::<i32>().ok();
            }
        } else if line.starts_with("Costo de la mejor solucion:") {
            if let Some(val) = line.split(':').nth(1) {
                costo = val.trim().parse::<f64>().ok();
            }
        } else if line.starts_with("Tiempo transcurrido:") {
            if let Some(val) = line.split(':').nth(1) {
                let v = val.trim().trim_end_matches('s');
                tiempo = v.parse::<f64>().ok();
            }
        }
    }

    if semilla.is_some() && tiempo.is_some() && costo.is_some() {
        Ok(Some(Reporte {
            semilla: semilla.unwrap(),
            tiempo: tiempo.unwrap(),
            costo: costo.unwrap(),
        }))
    } else {
        Ok(None)
    }
}

pub fn procesar_archivos() -> io::Result<()> {
    let carpeta = "./reports";
    let mut reportes = Vec::new();

    for entry in fs::read_dir(carpeta)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            if let Ok(Some(reporte)) = procesar_archivo(&path) {
                reportes.push(reporte);
            }
        }
    }

    if reportes.is_empty() {
        println!("No se encontraron reportes válidos en la carpeta.");
        return Ok(());
    }

    let mut por_costo = reportes.clone();
    por_costo.sort_by(|a, b| a.costo.partial_cmp(&b.costo).unwrap_or(Ordering::Equal));
    let mejores_costos: Vec<_> = por_costo.into_iter().take(10).collect();

    println!("\n=== Mejores 10 costos ===");
    for r in &mejores_costos {
        println!("Semilla: {}, Tiempo: {:.3}s, Costo: {:.6}", r.semilla, r.tiempo, r.costo);
    }

    Ok(())
}