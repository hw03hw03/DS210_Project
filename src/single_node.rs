use crate::Graph;
use crate::Vertex;

//a lot of this is also adapted from lecture 28, but I added the analytics part
pub fn single_node_distances(x:usize, graph:&Graph) -> (usize,usize,u32,f64) {
    use std::collections::VecDeque;
    let start: Vertex = x; // starting vertex
    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    distance[start] = Some(0);
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() { // new unprocessed vertex
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] { // consider all unprocessed neighbors of v
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    };
    let mut num_outgoing_edges = 0;
    let mut max_distance = 0;
    let mut total_distances = 0;
    for v in 0..graph.n {
        if distance[v].is_none(){
            continue
        }
        else{
            if distance[v].unwrap() > max_distance{
                max_distance = distance[v].unwrap();
            }
            num_outgoing_edges+=1;
            total_distances += distance[v].unwrap();
        }
    }
    num_outgoing_edges -= 1;//must subtract one because, by default, it counts the self connection as an outgoing edge
    let average_distance = total_distances as f64/num_outgoing_edges as f64;
    return (num_outgoing_edges,max_distance.try_into().unwrap(),total_distances,average_distance)

}

// creating a separate function to print the values returned in the other function so that I can reuse it to find all distances
pub fn single_node_analytics(x:usize,graph:&Graph) {
    let tuple_package = single_node_distances(x,graph);
    let num_outgoing_edges = tuple_package.0;
    let max_distance = tuple_package.1;
    let _total_distances = tuple_package.2;
    let average_distance = tuple_package.3;
    println!("----------------------------------------");
    println!("Starting at node: {}",x);
    println!("Total number of outgoing connections: {}", num_outgoing_edges); 
    println!("Maximum distance: {}",max_distance);
    println!("Average distance: {}",average_distance);
    println!("----------------------------------------");
}
