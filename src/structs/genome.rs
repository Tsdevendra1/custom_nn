use crate::structs::gene::{ConnectionGene, NodeGene};

pub struct Genome<'a> {
    pub id: i32,
    pub start_nodes: Vec<&'a NodeGene>,
    pub end_nodes: Vec<&'a NodeGene>,
    pub connections: Vec<ConnectionGene>,
    pub nodes: Vec<NodeGene>,
    pub fitness: f64,
}

//impl Genome {
//    fn get_all_paths(&self, ){
//
//    }
//}