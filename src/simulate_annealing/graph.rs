use crate::entity::city_with_distance::CityWithDistance;
use crate::entity::city::{City};

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: Vec<City>,
    edges: Vec<(f64,usize)>,
}

impl Graph {
    pub fn new(nodes: Vec<City>, edges: Vec<CityWithDistance>) -> Self {
        let mut edge_weights: Vec<(f64,usize)> = vec![(-1.0,0); nodes.len() * nodes.len()];
        for edge in edges.iter() {
            edge_weights[(edge.get_city() - 1) as usize * nodes.len() + (edge.get_origin() - 1) as usize] = (edge.distance_m, 1);
            edge_weights[(edge.get_origin() - 1) as usize * nodes.len() + (edge.get_city() - 1) as usize] = (edge.distance_m, 1);
        }
        Graph {
            nodes,
            edges: edge_weights,
        }
    }

    pub fn get_node_by_position(&self, position: usize) -> Option<&City> {
        self.nodes.get(position)
    }

    pub fn set_edge(&mut self, row: usize, column: usize, weight: f64) {
        if row < self.nodes.len() && column < self.nodes.len() {
            self.edges[row * self.nodes.len() + column] = (weight, 0);
        }
    }

    pub fn get_edge(&mut self, city_1: i32, city_2: i32) -> Option<(f64, usize)> {
        let row = (city_1 - 1) as usize;
        let column = (city_2 - 1) as usize;
        let edge = self.edges.get(row * self.nodes.len() + column);
        if let Some(edge) = edge {
            if edge.0 > -1.0 {
                return Some(*edge);
            } else {
                let city_a = self.get_node_by_position(row)?;
                let city_b = self.get_node_by_position(column)?;
                let distance = city_a.distance(city_b);
                self.set_edge(row, column, distance);
                self.set_edge(column, row, distance);
                return Some((distance, 0));
            }
        }
        None
    }
}
