use crate::structs::genome::Genome;
use crate::structs::gene::ConnectionGene;
use std::collections::HashSet;
use crate::maths_helpers::graph_algorithm::Graph;

pub(crate) struct DeconstructGenome {}

impl DeconstructGenome {
    fn unpack_genome(genome: Genome) {
        /// Unpacks the genome and returns information about it
        let nodes = &genome.nodes;
        let connections = &genome.connections;
        // todo: properly implement this
//        let sorted_connections =  DeconstructGenome::sort_connections(connections);
    }


    fn get_node_layers(connections: &[ConnectionGene]) {

        let enabled_connections = connections.iter().filter(|connection| {
            connection.enabled
        }).collect::<Vec<&ConnectionGene>>();
        let connections_as_tuples = enabled_connections.iter().map(|connection| {
            (connection.input_node,
             connection.output_node)
        }).collect::<Vec<(i32, i32)>>();

        let graph= Graph::new(&connections_as_tuples);


    }


    fn sort_connections(connections: &[ConnectionGene]) {}
}