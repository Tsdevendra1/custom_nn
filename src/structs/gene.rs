#[derive(Debug)]
pub struct ConnectionGene {
    pub enabled: bool,
    pub input_node: i32,
    pub output_node: i32,
    pub weight: f64,
    // Used in ghost nodes
    pub keep_constant_weight: bool,
}

impl ConnectionGene {
    fn new(enabled: bool, input_node: i32, output_node: i32, weight: f64, keep_constant_weight: bool)
           -> ConnectionGene {

        // Can't have a node which points to itself
        if input_node == output_node {
            panic!("input and output node cannot point to the same node")
        }

        ConnectionGene {
            enabled,
            input_node,
            output_node,
            weight,
            keep_constant_weight,
        }
    }
}

#[derive(Debug)]
pub enum NodeType {
    Source,
    Hidden,
    Output
}

#[derive(Debug)]
pub struct NodeGene {
    pub node_type: NodeType,
    pub node_id: i32,
    pub bias: Option<f64>,

}