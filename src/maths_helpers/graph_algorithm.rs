use std::collections::{HashMap, HashSet};
use std::borrow::Borrow;
use std::collections::hash_map::Entry::{Occupied, Vacant};

pub(crate) struct Graph {
    pub nodes: HashSet<i32>,
    pub connections: HashMap<i32, Vec<i32>>,
    pub node_layer_tracker: HashMap<i32, i32>,
}

impl Graph {
    pub(crate) fn new(connections: &[(i32, i32)]) -> Graph {
        let mut graph = Graph {
            nodes: HashSet::new(),
            connections: HashMap::new(),
            node_layer_tracker: HashMap::new(),
        };
        for connection in connections {
            graph.add_edge(connection.0, connection.1);
        }

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
    HashMap<i32, bool>, layer_number: &mut i32, path: &mut Vec<i32>, all_paths_tracker: &mut
    Vec<Vec<i32>>, node_layer_tracker: &mut HashMap<i32, i32>) {
        visited_tracker.insert(current_node, true);
        path.push(current_node);
        *layer_number += 1;

        if let Some(node_max_layer) = node_layer_tracker.get(&current_node) {
            if *layer_number > *node_max_layer {
                node_layer_tracker.insert(current_node, layer_number.clone());
            }
        } else {
            node_layer_tracker.insert(current_node, layer_number.clone());
        }

        if current_node == destination {
            all_paths_tracker.push(path.clone());
        } else if let Some(neighbours) = self.connections.get(&current_node) {
            for neighbour in neighbours {
                if !visited_tracker.get(neighbour).unwrap() {
                    self.explore_all_paths(*neighbour, destination, visited_tracker,
                                           layer_number, path, all_paths_tracker, node_layer_tracker);
                }
            }
        }
        *layer_number -= 1;
        path.pop();
        visited_tracker.insert(current_node, false);
    }

    pub fn get_all_paths(&mut self, start_nodes: &[i32], end_nodes: &[i32]) -> (Vec<Vec<i32>>, HashMap<i32, i32>) {
        let mut visited_tracker: HashMap<i32, bool>;
        // initalise as none of the nodes have been visited
        let mut all_paths_tracker = Vec::new();
        let mut node_layer_tracker = HashMap::new();

        for start_node in start_nodes {
            for end_node in end_nodes {
                visited_tracker = HashMap::new();
                for node in &self.nodes {
                    visited_tracker.insert(*node, false);
                }
                let mut layer_number = 0;
                let mut path = Vec::new();

                self.explore_all_paths(*start_node, *end_node, &mut visited_tracker, &mut
                    layer_number, &mut path, &mut all_paths_tracker, &mut node_layer_tracker);
            }
        }


        (all_paths_tracker, node_layer_tracker)
    }
}
