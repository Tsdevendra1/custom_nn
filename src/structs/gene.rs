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
#[derive(PartialEq)]
pub struct NodeGene {
    pub id: i32,
    pub bias: Option<f64>,
    pub node_type:  NodeType,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum NodeType {
    SourceNode,
    HiddenNode,
    OutputNode,
}


