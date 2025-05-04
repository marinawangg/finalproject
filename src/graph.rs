use petgraph::Undirected;
use petgraph::graph::{Graph, NodeIndex};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap,VecDeque};
use std::error::Error;
use crate::centrality::{central_finder, least_central_finder};

//this module contains the function to load the undirected graph from its csv format.
//and the struct and impl of Graphs.
//the methods associated with impl Graphs.
//this also contains the graph algorithms of BFS, Closeness Centrality, and Connected Components
//and the method to compare the reach of highly central and less central people/nodes.


//the Graphs struct represents a graph of the last.fm users. This graph is undirected,
//and is made up of u32 type node numbers.
pub struct Graphs {
    pub graph: Graph<u32, (), Undirected>,
}

//this constructor creates a new Graphs object from a petgraph.
impl Graphs {
    pub fn new(graph: Graph<u32, (), Undirected>) -> Graphs {
        Graphs{
            graph: graph,
        }
    }

    //this function computes bfs for a person for only a specified amount of steps.
    //input is the current graph (self), the start node, and the number of steps
    //output is a vector of visited nodes
    pub fn bfs(&self, num_begin: u32, steps: u32)-> Vec<u32> {
        let mut queue = VecDeque::new();
        let mut visited_nodes = Vec::new();
    
        let mut index_begin = None;
        //iterates over all node indices, finds the node with the value equal to num_begin,
        //and saves it as index_begin
        for node in self.graph.node_indices() {
            if self.graph[node] == num_begin {
                index_begin = Some(node);
                break;
            }
        }
        
        //adding the start node to the back of the queue
        let start_node = index_begin.unwrap();
        queue.push_back(start_node);
        visited_nodes.push(self.graph[start_node]);
    
        let mut distances: HashMap<NodeIndex, u32> = HashMap::new();
        distances.insert(start_node, 0);
        
        //loop over the nodes in the queue
        while let Some(node1) = queue.pop_front(){
            let dist_limit = distances[&node1];

            //if the maximum amount of steps was reached, skip the loop
            if dist_limit >= steps{
                continue;
            }

            let neighbors= self.graph.neighbors(node1);

            //for each unvisited neighbor, store the distance, mark it as visited, 
            // push it back in the queue
            for neighbor in neighbors {
                if !distances.contains_key(&neighbor){
                    distances.insert(neighbor, dist_limit+1);
                    //store the node number
                    visited_nodes.push(self.graph[neighbor]);
                    queue.push_back(neighbor);
                }
            }
        }
        visited_nodes
    }
    
    //this function computes the closeness centrality of a user,
    //how close they are to all the other users.
    //input is the current graph (self)
    //output is a hashmap of (node numbers, closeness centrality score) pairs
    pub fn closeness_centrality(&self) -> HashMap<u32, f64> {
        let mut map = HashMap::new();
        
        //for each node
        for node in self.graph.node_indices() {
            let mut distances = HashMap::new();
            let mut queue = VecDeque::new();
    
            distances.insert(node, 0);
            queue.push_back(node);
            
            //bfs is calculated to all nodes from a node
            while let Some(node1) = queue.pop_front() {
                let distance = distances[&node1];
                
                //for all neighbors of a node
                for neighbor in self.graph.neighbors(node1) {
                    if !distances.contains_key(&neighbor) {
                        distances.insert(neighbor, distance+1);
                        queue.push_back(neighbor);
                    }
                }
            }

            
            let sum: usize = distances.values().sum();
            let all_nodes = distances.len();

            //closeness centrality = the number of all other nodes/
            //the sum of all BFS distances to the other nodes
            if sum>0 {
                let score = (all_nodes-1) as f64/sum as f64;
                map.insert(self.graph[node],score);
            } else {
                map.insert(self.graph[node],0.0);
            }
        }
    
        map
    }
    
    //this function finds the connected components of a graph
    //takes the graph as &self
    //output is a list of connected components (a list of vectors of node numbers)


    pub fn connected_components(&self) -> Vec<Vec<u32>> {
        let mut ccomponents = Vec::new();
        let mut visited_nodes = Vec::new();
        
        //iterate over all nodes in the graph
        for node in self.graph.node_indices() {
            if !visited_nodes.contains(&node) {
                let mut component = Vec::new();
                let mut queue = VecDeque::new();
    
                queue.push_back(node);
                visited_nodes.push(node);
                
                //calculate BFS on all nodes of the component
                while let Some(node1) = queue.pop_front() {
                    let number = self.graph[node1];
                    component.push(number);
                    
                    //for each neighbor, add the unvisited ones to the queue
                    for neighbor in self.graph.neighbors(node1) {
                        if !visited_nodes.contains(&neighbor){
                            visited_nodes.push(neighbor);
                            queue.push_back(neighbor);
                        }
                    }
                }
                ccomponents.push(component);
            }
        }
        ccomponents
    }

    //how many last.fm users a person can reach within a given number of steps
    //input is the person, and number of steps and the graph as &self
    //output is the number of users a person can reach
    pub fn person_spread(&self, person: u32, steps: u32) -> usize {
        let mut queue = VecDeque::new();
        let mut visited_nodes = Vec::new();
        let mut distances= HashMap::new();
    
        let mut start_index = None;
        //find the NodeIndex that the given person has 
        for node in self.graph.node_indices() {
            if self.graph[node] == person {
                start_index = Some(node);
                break;
            }
        }
    
        let start = start_index.unwrap();

        queue.push_back(start);
        visited_nodes.push(start);
        distances.insert(start, 0);
        //BFS up to a certain number of steps
        while let Some(node1) = queue.pop_front() {
            let distance1 = *distances.get(&node1).expect("error getting distance");
    
            if distance1 < steps {
                for neighbor in self.graph.neighbors(node1) {
                    if !visited_nodes.contains(&neighbor) {
                        visited_nodes.push(neighbor);
                        distances.insert(neighbor, distance1+1);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    
        //number of nodes reached/number of users reached 
        visited_nodes.len()
    }

    //this function prints the reach of the most central users and the least central users
    //within a given amount of steps
    //the input is a list of connected components
    //a hash map of (node number, score) pairs, and the number of steps
    //the output prints the average number of users reached by the most and least central users
    pub fn high_vs_low_reach(&self,components: &Vec<Vec<u32>>, closeness_map: &HashMap<u32, f64>, steps: u32) {
        let most_central = central_finder(components, closeness_map);
        let least_central = least_central_finder(components, closeness_map);

        let mut most_sum = 0;
        let mut least_sum = 0;
        let num_components = most_central.len().min(least_central.len());

        //within each component, determine the number of users each can reach
        for i in 0..num_components {
            let most_reach = self.person_spread(most_central[i], steps);
            let least_reach = self.person_spread(least_central[i], steps);
            most_sum += most_reach;
            least_sum += least_reach;
        }

        let most_avg= most_sum as f64 / num_components as f64;
        let least_avg= least_sum as f64 / num_components as f64;

        println!("In {} steps, the most central users reached {:.2} people on average.", steps, most_avg);
        println!("In {} steps, the least central users reached {:.2} people on average.", steps, least_avg);
    }


}

//the input of this is a path to the csv file
//the output of this is an undirected petgraph::Graph
//each node is a u32, () since the graph is unweighted
pub fn read_graph(file_path: &str) -> Result<Graph<u32, (), Undirected>, Box<dyn Error>> {
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut graph = Graph::<u32, (), Undirected>::new_undirected();
    let mut nodes: HashMap<u32, NodeIndex> = HashMap::new();
    let mut line_num =0;


    for fline in reader.lines() {
        let line = fline.expect("Could not read line");
        if line_num ==0{
            line_num +=1;
            continue;
        }
        //to get 1000 entries
        if line_num >=1001{
            break;
        }
        
        let numbers: Vec<&str> = line.trim().split(',').collect();
        //parse the string values into u32 numbers
        let number1: u32 = numbers[0].parse().expect("error in parsing");
        let number2: u32 = numbers[1].parse().expect("error in parsing");

        // Add n1 as a node if it isn't already in the nodes HashMap
        let index1;
        if nodes.contains_key(&number1){
            index1= nodes[&number1];
        } else{
            let index= graph.add_node(number1);
            nodes.insert(number1, index);
            index1= index;
        }

        // Add n2 as a node if it isn't already in the nodes HashMap
        let index2;
        if nodes.contains_key(&number2) {
            index2 = nodes[&number2];
        } else {
        let index = graph.add_node(number2);
        nodes.insert(number2, index);
        index2 = index;
        }
        //Connecting the node indices using the .add_edge function in petgraph
        graph.add_edge(index1, index2, ());
        line_num+=1;

    }
    Ok(graph)
    }


#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::{Graph, NodeIndex};
    use petgraph::Undirected;
    use std::collections::HashMap;
    
    fn test_graph() -> Graphs {
        let mut graph = Graph::<u32, (), Undirected>::new_undirected();
        let a = graph.add_node(1);
        let b = graph.add_node(2);
        let c = graph.add_node(3);
        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());
        graph.add_edge(a, c, ());
        Graphs::new(graph)
    }
    #[test]
    fn test_bfs() {
        let graphs = test_graph();
        let result = graphs.bfs(1, 1);
        assert_eq!(result.len(), 3); 
        //bfs should reach 1, 2, and 3
    }
    
    #[test]
    fn test_closeness_centrality() {
        let graphs = test_graph();
        let centrality = graphs.closeness_centrality();
        assert_eq!(centrality.len(), 3); 
        //every node has a score
    }
    
    #[test]
    fn test_connected_components() {
        let graphs = test_graph();
        let components = graphs.connected_components();
        assert_eq!(components.len(), 1); 
        //there should be 1 connected group
    }
    
    #[test]
    fn test_person_spread() {
        let graphs = test_graph();
        let spread = graphs.person_spread(1, 1);
        assert_eq!(spread, 3); 
        //1 reaches 2 and 3
    }
    
    #[test]
    fn test_high_vs_low_reach() {
        let graphs = test_graph();
        let comps = graphs.connected_components();
        let closeness = graphs.closeness_centrality();
        graphs.high_vs_low_reach(&comps, &closeness, 1); 
        //testing that function works properly
    }



}