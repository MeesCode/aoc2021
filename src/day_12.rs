
use std::collections::{HashMap, HashSet};

pub fn main(){    
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();

    for i in include_str!("../data/day_12.txt").trim().lines() {
        let labels: Vec<&str> = i.split("-").collect();
        for i in [(1, 0), (0, 1)] {
            if nodes.contains_key(labels[i.0]) {
                nodes.get_mut(labels[i.0]).unwrap().push(labels[i.1]);
            } else {
                nodes.insert(labels[i.0], vec![labels[i.1]]);
            }
        }
    }
    
    let a = part_a(&nodes);
    println!("Part A result: {}", a);
    let b = part_b(&nodes);
    println!("Part B result: {}", b);
}

fn routes(nodes: &HashMap<&str, Vec<&str>>, current: &str, visited: &HashSet<&str>, done: bool) -> i32 {
    if current == "end" { return 1; }

    let upper: Vec<&str> = nodes.get(current).unwrap().iter().filter(|n| 
        n.chars().nth(0).unwrap().is_uppercase() ||
        !visited.contains(*n)
    ).map(|x| *x).collect();

    let lower: Vec<&str> = nodes.get(current).unwrap().iter().filter(|n| 
        n.chars().nth(0).unwrap().is_lowercase() &&
        visited.contains(*n) &&
        **n != "start" &&
        !done
    ).map(|x| *x).collect();

    let mut visited = visited.clone();
    visited.insert(current);

    upper.iter().fold(0, |a, x| a + routes(nodes, x, &visited, done)) + 
    lower.iter().fold(0, |a, x| a + routes(nodes, x, &visited, true))
}

fn part_a(nodes: &HashMap<&str, Vec<&str>>) -> i32 {
    routes(nodes, "start", &HashSet::new(), true)
}

fn part_b(nodes: &HashMap<&str, Vec<&str>>) -> i32 {
    routes(nodes, "start", &HashSet::new(), false)
}