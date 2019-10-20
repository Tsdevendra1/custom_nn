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
        for connection in connections {}

        graph
    }

    fn add_edge(&mut self, start_node: i32, end_node: i32) {
        match self.connections.get_mut(start_node.borrow()) {
            Some(connections) => {
                connections.push(end_node);
            }
            None => {
                self.connections.insert(start_node, vec![end_node]);
            }
        }

        self.nodes.insert(start_node);
        self.nodes.insert(end_node);
    }

    fn explore_all_paths(&self, current_node: i32, destination: i32, visited_tracker: &mut
    HashMap<i32,
        bool>, path_count: i32, layer_number: &mut i32, path: &mut Vec<i32>,
                         all_paths_tracker: &mut Vec<Vec<i32>>) {
        visited_tracker.insert(current_node, true);
        path.push(current_node);
        *layer_number += 1;

        if current_node == destination {
            all_paths_tracker.push(path.clone());
            *layer_number -= 1;
            path.pop();
            visited_tracker.insert(current_node, false);
        } else {
            if let Some(neighbours) = self.connections.get(&current_node) {
                for neighbour in neighbours {
                    if !visited_tracker.get(neighbour).unwrap() {
                        self.explore_all_paths(current_node, destination, visited_tracker,
                                               path_count, layer_number, path, all_paths_tracker);
                    }
                }
            }
        }
    }

    fn get_all_paths(&self, start_node: i32, end_node: i32) {
        let mut visited_tracker = HashMap::new();
        // initalise as none of the nodes have been visited
        for node in &self.nodes {
            visited_tracker.insert(node, false);
        }
    }
}