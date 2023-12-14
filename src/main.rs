mod read;
mod single_node;
mod all_node;
mod graph_from_read;

fn main() {
    use crate::graph_from_read::make_graph_from_read;
    let graph = make_graph_from_read("src/list_of_edges.csv");
    use crate::single_node::single_node_analytics;
    use crate::all_node::all_node_analytics;
    single_node_analytics(0,&graph);
    single_node_analytics(1,&graph);
    single_node_analytics(8,&graph);
    all_node_analytics(&graph);
}

// the types, struct, and impl below are adapted from lecture_28 for use in my directed graph. the graph struct must be public because it is used in other modules
type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;
#[derive(Debug)]
pub struct Graph {
    n: usize, // vertex labels in {0,...,n-1}
    outedges: AdjacencyLists,
}
impl Graph {
    fn add_directed_edges(&mut self, edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n:usize,edges:&ListOfEdges) -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
}

#[test]
fn test_grandparent_node(){
    use crate::graph_from_read::make_graph_from_read;
    let graph = make_graph_from_read("src/list_of_edges.csv");
    use crate::single_node::single_node_distances;
    let grandparent_outgoing_connections = single_node_distances(0,&graph).0;
    assert_eq!(grandparent_outgoing_connections,graph.n-1,"Grandparent node is not connected to all other nodes -- something unexpected has happened.");
}

#[test]
fn test_max_connection(){
    use crate::graph_from_read::make_graph_from_read;
    let graph = make_graph_from_read("src/list_of_edges.csv");
    use crate::single_node::single_node_distances;
    let max_distance = &graph.n;
    for i in 0..graph.n {
        assert!(single_node_distances(i, &graph).0 < graph.n,"Some node has more connections than there are nodes.")
    }
}