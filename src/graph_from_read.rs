use crate::Graph;

pub fn make_graph_from_read(path: &str) -> Graph {
    use crate::read::read_file;
    let read_output: (Vec<(usize,usize)>,usize) = read_file(path);
    let mut edges = read_output.0;
    let num_unique_nodes = read_output.1;
    edges.sort();
    let temp_graph = Graph::create_directed(num_unique_nodes,&edges);
    return temp_graph
}