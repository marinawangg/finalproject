mod graph;
mod centrality;
use graph::{Graphs,read_graph};
use crate::centrality::{most_central_user, least_central_user, largest_component};

//in the main.rs module, I have my main function 
//where I run and print out the results to answer my question.

fn main(){
    //loading the graph
    let graph = read_graph("edges/lastfm_asia_edges.csv").expect("Csv not found");
    let data_graph= Graphs::new(graph);

    //running connected components, finding the largest component
    let components = data_graph.connected_components();
    let largest = largest_component(&components);

    //running closeness centrality
    let cl_cn = data_graph.closeness_centrality();

    //finding the relationship between closeness centrality and reach within 3 steps
    data_graph.high_vs_low_reach(&components, &cl_cn, 3);

    //bfs of the most central user in the largest component
    let high = most_central_user(&largest, &cl_cn);
    let high_bfs = data_graph.bfs(high,3);
    
    //bfs of the least central user in the largest component
    let low= least_central_user(&largest, &cl_cn);
    let low_bfs = data_graph.bfs(low,3);

    println!("The most central user reached {} people during BFS in 3 steps", high_bfs.len());
    println!("The least central user reached {} people during BFS in 3 steps", low_bfs.len());
}

