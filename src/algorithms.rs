use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::{HashMap, VecDeque};

pub fn bfs(graph: &Graph<u32, (), Undirected>, num_begin: u32) {
    let mut queue = VecDeque::new();
    let mut map = Vec::new();

    let mut index_begin = None;
    for node in graph.node_indices() {
        if graph[node] == num_begin {
            index_begin = Some(node);
            break;
        }
    }
//comments finish
    if index_begin.is_none() {
        println!("invalid starting node");
        return;
    }

    let start_node = index_begin.unwrap();
    queue.push_back(start_node);
    map.push(start_node);

    let mut distances: HashMap<NodeIndex, u32> = HashMap::new();
    distances.insert(start_node, 0);

    while let Some(node1) = queue.pop_front(){
        let num_current = graph[node1];

        for neighbor in graph.neighbors(node1) {
            if !map.contains(&neighbor){
                let dist_new = distances[&node1] + 1;
                distances.insert(neighbor, dist_new);
                map.push(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
    map
}

pub fn closeness_centrality(graph: &Graph<u32, (), Undirected>) -> HashMap<u32, f64> {
    let mut map = HashMap::new();

    for node in graph.node_indices() {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();

        distances.insert(node, 0);
        queue.push_back(node);

        while let Some(node1) = queue.pop_front() {
            let distance = distances[&node1];

            for neighbor in graph.neighbors(node1) {
                if !distances.contains_key(&neighbor) {
                    distances.insert(neighbor, distance+1);
                    queue.push_back(neighbor);
                }
            }
        }

        let sum: usize = distances.values().sum();
        let all_nodes = distances.len();

        if sum>0 {
            let score = (all_nodes-1) as f64/sum as f64;
            map.insert(graph[node],score);
        } else {
            map.insert(graph[node],0.0);
        }
    }

    map
}

pub fn connected_components(){
//start tomorrow


}