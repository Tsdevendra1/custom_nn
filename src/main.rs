use std::collections::HashMap;

mod maths_helpers;
use crate::maths_helpers::graph_algorithm::Graph;


fn main(){
    println!("h√k`√i");
    let connections = vec![(1, 2), (2, 3), (3, 4)];
    let graph = Graph::new(&connections);
    println!("{:?}", graph.connections);
    let paths = graph.get_all_paths(&vec![1], &vec![4]);
    println!("{:?}", paths);
}