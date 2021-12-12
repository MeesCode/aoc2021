
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

fn routes(nodes: &HashMap<&str, Vec<&str>>, current: &str, visited: &HashSet<&str>, cur_path: &Vec<&str>, paths: &mut HashSet<Vec<String>>, double: &str, done: bool) -> i32 {
    if current == "end" { 
        paths.insert(cur_path.iter().map(|x| String::from(*x)).collect());
        return 1; 
    }

    let next: Vec<&str> = nodes.get(current).unwrap().iter().filter(|n| 
        n.chars().nth(0).unwrap().is_uppercase() || 
        !visited.contains(*n) ||
        (&double == *n && visited.contains(*n) && !done)
    ).map(|x| *x).collect();

    let new_done = done || (double == current && visited.contains(current));

    let mut new_visited = visited.clone();
    new_visited.insert(current);

    let mut cur_path = cur_path.clone();
    cur_path.push(current);

    next.iter().fold(0, |a, x| a + routes(nodes, x, &new_visited, &cur_path, paths, double, new_done))
}

fn part_a(nodes: &HashMap<&str, Vec<&str>>) -> i32 {
    routes(nodes, "start", &HashSet::new(), &Vec::new(), &mut HashSet::new(), "", true)
}

fn part_b(nodes: &HashMap<&str, Vec<&str>>) -> i32 {
    let mut paths: HashSet<Vec<String>> = HashSet::new();

    for i in nodes.keys() {
        if *i == "start" || *i == "end" || i.chars().nth(0).unwrap().is_uppercase() { continue; }
        routes(nodes, "start", &HashSet::new(), &Vec::new(), &mut paths, i, false);
    }

    paths.len() as i32
}