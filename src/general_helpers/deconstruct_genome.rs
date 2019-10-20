use crate::structs::genome::Genome;
use crate::structs::gene::ConnectionGene;

pub(crate) struct DeconstructGenome {}

impl DeconstructGenome {
    fn unpack_genome(genome: Genome) {
        /// Unpacks the genome and returns information about it
        let nodes = &genome.nodes;
        let connections = &genome.connections;
        let sorted_connections =  DeconstructGenome::sort_connections(connections);
    }

    fn sort_connections(connections: &Vec<ConnectionGene>){

    }
}