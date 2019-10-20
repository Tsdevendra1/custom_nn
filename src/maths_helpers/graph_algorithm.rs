use std::collections::{HashMap, HashSet};
use std::borrow::Borrow;
use crate::structs::gene::ConnectionGene;

pub struct Graph {
    nodes: HashSet<i32>,
    connections: HashMap<i32, Vec<i32>>,
}

impl Graph {
    fn new(connections: &Vec<ConnectionGene>) -> Graph {
        let graph = Graph { nodes: HashSet::new(), connections: HashMap::new() };
        for connection in connections {

        }

        graph
    }

    fn add_edge(&mut self, start_node: i32, end_node: i32) {

        match self.connections.get_mut(start_node.borrow()) {
            Some(connections) => {
                connections.push(end_node);
            },
            None => {
                self.connections.insert(start_node, vec![end_node]);
            }
        }

        self.nodes.insert(start_node);
        self.nodes.insert(end_node);

    }
}