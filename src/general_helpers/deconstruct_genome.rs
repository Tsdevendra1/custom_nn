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
//        let sorted_connections =  DeconstructGenome::sort_connections(connections);
    }


    fn get_node_layers<'a>(connections: &[ConnectionGene], start_nodes: &[&NodeGene],
                       end_nodes: &[&NodeGene]) -> HashMap<i32,i32> {

        let enabled_connections = connections.iter().filter(|connection| {
            connection.enabled
        }).collect::<Vec<&ConnectionGene>>();
        let connections_as_tuples = enabled_connections.iter().map(|connection| {
            (connection.input_node,
             connection.output_node)
        }).collect::<Vec<(i32, i32)>>();

        let mut graph= Graph::new(&connections_as_tuples);

        let start_node_ids = start_nodes.iter().map(|node|{node.id}).collect::<Vec<i32>>();
        let end_node_ids = end_nodes.iter().map(|node|{node.id}).collect::<Vec<i32>>();

        let (_, node_layers) = graph.get_all_paths(&start_node_ids, &end_node_ids);

        return node_layers


    }


    fn sort_connections(connections: &[ConnectionGene]) {}
}