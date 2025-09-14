use crate::simulate_annealing::path::Path;
use rand::{SeedableRng};
use rand::rngs::StdRng;

pub struct InitialTemperature {
    temperature : f64,
    percentage : f64,
    e_p : f64,
    path : Path,
    n : usize,
    random : StdRng
}

impl InitialTemperature  {
    pub fn new(
        temperature : f64,
        percentage : f64,
        e_p : f64,
        path : Path,
        n : usize,
        seed : u64
    ) -> Self {
        InitialTemperature { 
            temperature, 
            percentage,
            e_p, 
            path, 
            n,
            random: StdRng::seed_from_u64(seed)
        }
    }

    pub fn get_initial_t(&mut self, limit : usize) -> f64{
        let mut p = self.accept_percentage();
        if (self.percentage - p).abs() <= self.e_p {
            return self.temperature;
        }

        let t1 ;
        let t2 ;
        let i: usize = 0;
        if p < self.percentage {
            while p < self.percentage {
                if i == limit {
                    break;
                }
                self.temperature *= 2.0;
                p = self.accept_percentage();
            }
            
            t1 = self.temperature / 2.0;
            t2 = self.temperature;
        } else {
            while p > self.percentage {
                if i == limit {
                    break;
                }
                self.temperature /= 2.0;
                p = self.accept_percentage();
            }

            t1 = self.temperature;
            t2 = self.temperature * 2.0;
        }

        self.binary_search(t1,t2)
    }

    fn accept_percentage(&mut self) -> f64{
        let mut c = 0.0;
        for _ in 0..self.n {
            let vecino = self.path.get_vecino(&mut self.random);
            if vecino.2 <= self.path.get_cost() + self.temperature {
                self.path.apply_vecino();
                c += 1.0;
            }
        }
        c / self.n as f64
    }

    fn binary_search(&mut self, t_1 : f64, t_2 : f64) -> f64 {
        let t_m = (t_1 + t_2) / 2.0;
        if t_2 - t_1 < self.e_p {
            return t_m;
        }

        let p = self.accept_percentage();
        if (self.percentage - p).abs() < self.e_p {
            return t_m;
        } 
        if p > self.percentage {
            return self.binary_search(t_1, t_m);
        }

        self.binary_search(t_m, t_2)
    } 
}