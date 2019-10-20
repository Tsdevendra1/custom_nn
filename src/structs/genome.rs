use crate::structs::gene::{ConnectionGene, NodeGene};

pub struct Genome {
    pub id: i32,
    pub connections: Vec<ConnectionGene>,
    pub nodes: Vec<NodeGene>,
    pub fitness: f64,
}

impl Genome {
    fn get_all_paths(&self, ){

    }
}