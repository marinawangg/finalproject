use std::collections::HashMap;
//the centrality.rs module contains functions relating to the computation of centrality

//This function finds the most central user in each connected component
//Input is a list of connected components (a list of node numbers)
//and a HashMap that contains (user, closeness centrality score) pairs.
//Output is a vector containing the most central nodes from each component
pub fn central_finder(components: &Vec<Vec<u32>>, closeness_map: &HashMap<u32, f64>,) -> Vec<u32> {
    let mut central_users = Vec::new();

    //for each component, find the user with the highest closeness score.
    for component in components {
        let mut most_central_user = component[0];
        let mut highest_score = *closeness_map.get(&most_central_user).expect("error in finding central user");

        for &user in component {
            if let Some(&score) = closeness_map.get(&user) {
                if score > highest_score {
                    most_central_user = user;
                    highest_score = score;
                }
            } 
        }
        central_users.push(most_central_user);
    }
    central_users
}

//This function finds the least central user in each connected component.
//Input is a list of connected components (a list of node numbers)
//and a HashMap that contains (user, closeness centrality score) pairs.
//Output is a vector containing the least central nodes from each component
pub fn least_central_finder(components: &Vec<Vec<u32>>, closeness_map: &HashMap<u32, f64>) -> Vec<u32> {
    let mut least_central_users = Vec::new();
    for component in components {
        let mut least_central_user = component[0];
        let mut lowest_score = *closeness_map.get(&least_central_user).expect("error in finding central user");

        //for each component, find the user with the lowest closeness score.
        for &user in component {
            if let Some(&score) = closeness_map.get(&user) {
                if score < lowest_score {
                    least_central_user = user;
                    lowest_score = score;
                }
            }
        }
        least_central_users.push(least_central_user);
    }
    least_central_users
}

//this function finds the most central user in one component.
//inputs: a vector of node numbers in one connected component and a HashMap that contains (user, closeness centrality score) pairs.
//output: node number with the highest centrality score 
pub fn most_central_user(component: &Vec<u32>,closeness_map: &HashMap<u32, f64>,) -> u32 {
    let mut most_central = component[0];
    //dereference the reference to the f64 type
    let mut highest_score = *closeness_map.get(&most_central).expect("User not found");

    for user in component{
        let score = *closeness_map.get(user).expect("User not found");
        if score > highest_score {
            most_central = *user;
            highest_score = score;
        }
    }
    most_central
}

//this function finds the least central user in one component.
//inputs: a vector of node numbers in one connected component and a HashMap that contains (user, closeness centrality score) pairs.
//output: node number with the lowest centrality score 
pub fn least_central_user(component: &Vec<u32>,closeness_map: &HashMap<u32, f64>,) -> u32 {
    let mut least_central_user = component[0];
    //dereference the reference to the f64 type
    let mut lowest_score = *closeness_map.get(&least_central_user).expect("User not found");

    for user in component {
        let score = *closeness_map.get(user).expect("User not found");
        if score < lowest_score {
            least_central_user = *user;
            lowest_score = score;
        }
    }
    least_central_user
}

//This function returns the largest component determined by number of users given a list of components.
pub fn largest_component(components: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut largest = &components[0];

    for component in components {
        if component.len() > largest.len() {
            largest = component;
        }
    }

    largest.clone() //since largest is a reference within components
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_central_finder() {
        let components = vec![vec![1, 2, 3]];
        let mut scores = HashMap::new();
        scores.insert(1, 0.2);
        scores.insert(2, 0.5); //most central
        scores.insert(3, 0.1);
        let result = central_finder(&components, &scores);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_least_central_finder() {
        let components = vec![vec![1, 2, 3]];
        let mut scores = HashMap::new();
        scores.insert(1, 0.2);
        scores.insert(2, 0.5);
        scores.insert(3, 0.1); //least central
        let result = least_central_finder(&components, &scores);
        assert_eq!(result, vec![3]);
    }

    #[test]
    fn test_most_central_user() {
        let component = vec![1, 2, 3];
        let mut scores = HashMap::new();
        scores.insert(1, 0.2);
        scores.insert(2, 0.5); //most central
        scores.insert(3, 0.1);
        let result = most_central_user(&component, &scores);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_least_central_user() {
        let component = vec![1, 2, 3];
        let mut scores = HashMap::new();
        scores.insert(1, 0.2);
        scores.insert(2, 0.5);
        scores.insert(3, 0.1); //least central
        let result = least_central_user(&component, &scores);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_largest_component() {
        let components = vec![vec![1, 2, 3],vec![4],vec![5, 6],];
        let result = largest_component(&components);
        assert_eq!(result, vec![1, 2, 3]);
    }  
}