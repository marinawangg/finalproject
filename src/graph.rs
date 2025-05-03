use petgraph::Undirected;
use petgraph::graph::{NodeIndex, UnGraph};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use csv::Reader;
use std::error::Error;


pub fn read_graph(file_path: &str) -> Result<Graph<u32, (), Undirected>, Box<dyn Error>> {
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut graph = Graph::<u32, (), Undirected>::new_undirected();
    let mut nodes: HashMap<u32, NodeIndex> = HashMap::new();

    for fline in reader.lines() {
        let line = fline.expect("Could not read line");
        if line.trim().is_empty(){
            continue;
        }
        
        let numbers: Vec<&str> = line.trim().split(',').collect();
        //parse the string values into u32 numbers
        let number1: u32 = numbers[0].parse().expect("error in parsing");
        let number2: u32 = numbers[1].parse().expect("error in parsing");

        // Add n1 as a node if it isn't already in nodes
        let index1;
        if nodes.contains_key(&number1){
            index1= nodes[&number1];
        } else{
            let index= graph.add_node(number1);
            nodes.insert(number1, index);
            index1= index;
        }

        let index2;
        if nodes.contains_key(&number2) {
            index2 = nodes[&number2];
        } else {
        let index = graph.add_node(number2);
        nodes.insert(number2, index);
        index2 = index;
        }

        graph.add_edge(index1, index2, ());
    }
    Ok(graph)
}