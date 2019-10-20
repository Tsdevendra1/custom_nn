use crate::maths_helpers::graph_algorithm::Graph;
#[cfg(test)]
mod tests {
    use crate::maths_helpers::graph_algorithm::Graph;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn test_graph_creation() {
        let connections = vec![(1, 2), (2, 3), (3, 4)];
        let graph = Graph::new(&connections);
        println!("{:?}", graph.connections);
//        let paths = graph.get_all_paths(&vec![1], &vec![4]);
//        println!("{:?}", paths);

    }
}

mod structs;
mod general_helpers;
mod maths_helpers;


