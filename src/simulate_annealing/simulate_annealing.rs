
use crate::simulate_annealing::path::Path;
use crate::utils::generate_svg::GenerateSvg;
use rand::{SeedableRng};
use rand::rngs::StdRng;

pub struct SimulatedAnnealing {
    initial_temperature: f64,
    cooling_rate: f64,
    current_solution: Path,
    best_solution : Path,
    size_lote: usize,
    random: StdRng,
    e: f64,
    limit: usize
}

impl SimulatedAnnealing {
    pub fn new(
        initial_temperature: f64,
        cooling_rate: f64,
        current_solution: Path,
        size_lote: usize,
        seed: u64,
        e: f64,
        limit: usize
    ) -> Self {
        let best_solution = current_solution.clone();
        SimulatedAnnealing {
            initial_temperature,
            cooling_rate,
            current_solution,
            best_solution : best_solution,
            size_lote,
            random: StdRng::seed_from_u64(seed),
            e: e,
            limit: limit
        }
    }

    fn calculate_lote(&mut self,  generate_svg : &mut GenerateSvg) -> (f64,usize){
        let mut cost = 0.0;
        let mut c: usize = 0;
        let mut i = 0;
        while c <= self.size_lote {
            if i == self.size_lote * self.limit {
                break;
            }
            let vecino = self.current_solution.get_vecino(&mut self.random);
            generate_svg.add_point((vecino.2,0));
            if vecino.2 <= self.current_solution.get_cost() + self.initial_temperature {
                self.current_solution.apply_vecino();
                println!("{}",self.current_solution.get_cost());
                c += 1;
                cost += self.current_solution.get_cost();
                if self.current_solution.get_cost() < self.best_solution.get_cost() {
                    self.best_solution = self.current_solution.clone();
                    generate_svg.add_point((vecino.2,1));
                }
            }
            i += 1;
         }
         (cost / self.size_lote as f64,i)
    }

    pub fn accept_threshold(&mut self, generate_svg : &mut GenerateSvg) -> usize{
        let mut p = 0.0;
        let mut total = 0;
        while self.initial_temperature > self.e {
            let mut q: f64 = f64::MAX;
            while p <= q {
                q = p;
                let (new_p,i) = self.calculate_lote(generate_svg);
                p = new_p;
                total += i;
                if i == self.size_lote * self.limit {
                    return total;
                }
            }
            self.initial_temperature *= self.cooling_rate;
        }

        total 
    }

    pub fn get_best_solution(&mut self) -> &Path {
        self.current_solution.get_min();
        self.current_solution.apply_vecino();
        &self.current_solution
    }
}
