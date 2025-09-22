use std::vec;

use rand::{Rng, rngs::StdRng};
use crate::{simulate_annealing::graph::Graph};
#[derive(Clone)]
pub struct Path {
    path: Vec<i32>,
    cost: f64,
    graph: Graph,
    normalize : f64,
    vecino : (usize,usize,f64),
    distance_max : f64,
}

impl Path {
    pub fn new(path: Vec<i32>, graph: Graph) -> Self {
        Path { path, cost: 0.0, graph, normalize: 0.0, vecino: (0, 0, 0.0), distance_max : 0.0 }
    }

    pub fn get_path(&self) -> &Vec<i32> {
        &self.path
    }

    pub fn get_cost(&mut self) -> f64 {
        if self.normalize == 0.0 {
            (self.normalize,self.distance_max) = self.calculate_normalization();
        }

        if self.cost == 0.0 {
            self.cost = self.calculate_cost(self.normalize,self.distance_max);
        }
        self.cost
    }

    pub fn get_normalize(&mut self) -> f64 {
        if self.normalize == 0.0 {
            (self.normalize,self.distance_max) = self.calculate_normalization();
        }
        self.normalize
    }

    pub fn get_distance_max(&mut self) -> f64 {
        if self.normalize == 0.0 {
            (self.normalize,self.distance_max) = self.calculate_normalization();
        }

        self.distance_max
    }

    pub fn get_vecino(&mut self, random: &mut StdRng) -> (usize, usize, f64) {
        if self.path.len() < 2 {
            panic!("No se puede generar vecino con menos de 2 elementos en path");
        }

        let index_1 = random.gen_range(1..self.path.len());
        let index_2 = random.gen_range(0..index_1);     

        if index_1 == index_2 {
            return self.get_vecino(random); 
        }

        self.vecino = self.calculate_vecino(index_1, index_2);
        self.vecino
    }

    pub fn apply_vecino(&mut self) -> bool {
        if self.vecino.2 == 0.0 {
            return false;
        }

        self.path.swap(self.vecino.0, self.vecino.1);
        self.cost = self.vecino.2;
        self.vecino = (0, 0, 0.0);
        return true;
    }

    pub fn get_min(&mut self) -> (usize,usize,f64){
        let cost = self.get_cost();
        println!("{}", cost);
        for i in 0..self.path.len() {
            for j in i+1..self.path.len() {
                let vecino = self.calculate_vecino(i, j);
                if vecino.2 < cost {
                    self.vecino = vecino;
                    self.apply_vecino();
                    return self.get_min();
                }
            }
        }

        self.vecino
    }

    pub fn clone(&self) -> Self {
        Path {
            path: self.path.clone(),
            cost: self.cost,
            graph: self.graph.clone(),
            normalize: self.normalize,
            vecino: self.vecino,
            distance_max: self.distance_max,
        }
    }

    fn calculate_vecino(&mut self,index_1 : usize , index_2 : usize) -> (usize,usize,f64) {
        self.path.swap(index_1, index_2);
        let (normalize,distance_max) = (self.get_normalize(),self.get_distance_max());
        let cost = self.calculate_cost(normalize,distance_max);
        self.path.swap(index_1, index_2);
        (index_1, index_2, cost)
    }

    fn calculate_normalization(&mut self) -> (f64,f64) {
        let mut edges: Vec<f64> = vec![];
        for i in 0..self.path.len() {
            for j in i+1..self.path.len() {
                if let Some(weight) = self.graph.get_edge(self.path[i], self.path[j]) && weight.1 == 1 {
                        edges.push(weight.0);
                }
            }   
        }
        edges.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let distance_max = edges[0];
        let normalization: f64 = edges.iter().take(self.path.len()-1).sum();
        (normalization,distance_max)
    }

    fn calculate_cost(&mut self, normalization: f64,distance_max : f64) -> f64 {
        let mut cost = 0.0;
        for i in 0..(self.path.len() - 1) {
            if let Some(mut weight) = self.graph.get_edge(self.path[i], self.path[i + 1]) {
                if weight.1 == 0 {
                    weight.0 *= distance_max;
                }
                cost += weight.0;
            }
        }
        cost / normalization
    }
}