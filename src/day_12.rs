
use std::collections::{HashMap, HashSet};

pub fn main(){    
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();

    for i in include_str!("../data/day_12.txt").trim().lines() {
        let labels: Vec<&str> = i.split("-").collect();

        if nodes.contains_key(labels[0]) {
            nodes.get_mut(labels[0]).unwrap().push(labels[1]);
        } else {
            nodes.insert(labels[0], vec![labels[1]]);
        }

        if nodes.contains_key(labels[1]) {
            nodes.get_mut(labels[1]).unwrap().push(labels[0]);
        } else {
            nodes.insert(labels[1], vec![labels[0]]);
        }
    }
    
    let a = part_a(&nodes);
    println!("Part A result: {}", a);
    // let b = part_b(&input);
    // println!("Part B result: {}", b);
}

fn routes(nodes: &HashMap<&str, Vec<&str>>, current: &str, visited: &HashSet<&str>) -> i32 {
    if current == "end" { return 1; }

    let paths: Vec<&str> = nodes.get(current).unwrap().iter().filter(|n| !(n.chars().nth(0).unwrap().is_lowercase() && visited.contains(*n))).map(|x| *x).collect();
    if paths.len() == 0 { return 0; }

    let mut new_visited = visited.clone();
    new_visited.insert(current);

    paths.iter().fold(0, |a, x| a + routes(nodes, x, &new_visited))
}

fn part_a(nodes: &HashMap<&str, Vec<&str>>) -> i32 {

    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert("start");

    let routes = routes(nodes, "start", &visited);

    routes
}