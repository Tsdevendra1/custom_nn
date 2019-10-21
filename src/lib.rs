use crate::maths_helpers::graph_algorithm::Graph;
#[cfg(test)]
mod tests {
    use crate::maths_helpers::graph_algorithm::Graph;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn test_graph_simple() {
        let connections = vec![(1, 2), (2, 3), (3, 4)];
        let mut graph = Graph::new(&connections);
        let paths = graph.get_all_paths(&vec![1], &vec![4]);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].len(), 4);
    }

    #[test]
    fn test_graph_more_complex() {
        let connections = vec![(1, 2), (2, 3), (3, 4), (4,5), (1,5), (6,5)];
        let mut graph = Graph::new(&connections);
        let paths = graph.get_all_paths(&vec![1, 6], &vec![5]);
        println!("{:?}", graph.node_layer_tracker);
        assert_eq!(paths.len(), 3);
        assert_eq!(paths[0].len(), 5);
        assert_eq!(paths[1].len(), 2);
        assert_eq!(paths[2].len(), 2);
    }
}

mod structs;
mod general_helpers;
mod maths_helpers;


