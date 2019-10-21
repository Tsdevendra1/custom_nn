use crate::structs::genome::Genome;
use crate::structs::gene::{ConnectionGene, NodeGene};
use std::collections::{HashSet, HashMap};
use crate::maths_helpers::graph_algorithm::Graph;


pub(crate) struct DeconstructGenome {}

impl DeconstructGenome {
    fn unpack_genome(genome: Genome) {
        /// Unpacks the genome and returns information about it
        let nodes = &genome.nodes;
        let connections = &genome.connections;
        // todo: properly implement this

        let node_layers = DeconstructGenome::get_node_layers(connections, &genome.start_nodes,
                                                             &genome.end_nodes);

        let num_layers = node_layers.values().max().unwrap();
        let layer_nodes = DeconstructGenome::get_nodes_per_layer(&node_layers);

        let node_map = DeconstructGenome::get_node_map(&layer_nodes);
    }


    fn get_node_layers(connections: &[ConnectionGene], start_nodes: &[&NodeGene],
                       end_nodes: &[&NodeGene]) -> HashMap<i32, i32> {
        let enabled_connections = connections.iter().filter(|connection| {
            connection.enabled
        }).collect::<Vec<&ConnectionGene>>();
        let connections_as_tuples = enabled_connections.iter().map(|connection| {
            (connection.input_node,
             connection.output_node)
        }).collect::<Vec<(i32, i32)>>();

        let mut graph = Graph::new(&connections_as_tuples);

        let start_node_ids = start_nodes.iter().map(|node| { node.id }).collect::<Vec<i32>>();
        let end_node_ids = end_nodes.iter().map(|node| { node.id }).collect::<Vec<i32>>();

        let (_, node_layers) = graph.get_all_paths(&start_node_ids, &end_node_ids);

        node_layers
    }

    fn get_nodes_per_layer(node_layers: &HashMap<i32, i32>) -> HashMap<&i32, Vec<&i32>> {
        /// Gets a vec of nodes which are in each layer
        let mut layer_nodes: HashMap<&i32, Vec<&i32>> = HashMap::new();
        for (node_id, layer) in node_layers {
            layer_nodes.entry(layer).or_insert(vec![node_id]).push(node_id);
        }
        layer_nodes
    }


    fn get_node_map(layer_nodes: &HashMap<&i32, Vec<&i32>>) {

    }
}