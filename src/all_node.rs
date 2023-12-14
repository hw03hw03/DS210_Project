use crate::Graph;

pub fn all_node_analytics(graph: &Graph){
    use crate::single_node::single_node_distances;
    let mut num_nodes_with_connection = 0; // this variable will show us how many nodes have outgoing connections
    let mut total_average_shortest_distances = 0 as f64;
    for i in 0..graph.n {
        let mut current_total_average_dist = 0.0 as f64;
        //println!("Distances from node {}", i);
        current_total_average_dist += single_node_distances(i, &graph).3;
        //println!("{:?}",current_total_average_dist);
        if current_total_average_dist > 0.0 as f64 {
            num_nodes_with_connection += 1;
            //println!("HERE current_total_average_dist is: {}", current_total_average_dist);
            total_average_shortest_distances += current_total_average_dist as f64;
            //println!("HERE total_average_shortest_distances: {}", total_average_shortest_distances);
        }
    }
    num_nodes_with_connection -= 1; //same as other mods, need to subtract one because it counts itself as a node with a connection (because dist is 0 not None)
    println!("The average of average shortest distances for nodes WITH CONNECTIONS is: {}", total_average_shortest_distances/num_nodes_with_connection as f64);
    println!("The number of nodes WITH outgoing connections is: {}", num_nodes_with_connection);
    println!("The number of nodes WITHOUT an outgoing connection is: {}",graph.n - num_nodes_with_connection);
}
