
use std::collections::HashMap;

pub fn main(){    
    let mut input = include_str!("../data/day_14.txt").trim().lines();
    
    let template: Vec<char> = input.next().unwrap().chars().collect();
    input.next();

    let mut insertions = HashMap::new();
    for i in input {
        let parts: Vec<&str> = i.split(" -> ").collect();
        insertions.insert((parts[0].chars().nth(0).unwrap(), parts[0].chars().nth(1).unwrap()), parts[1].chars().nth(0).unwrap());
    }

    let a = part_a(&template, &insertions);
    println!("Part 1: {}", a);
    // let b = part_b(&input);
    // println!("Part 2: {}", b);
}

fn part_a(template: &Vec<char>, insertions: &HashMap<(char, char), char>) -> i32 {

    let mut template = template.clone();

    for _step in 1..41 {
        
        let mut i = 0;
        while i < template.len()-1 {

            if let Some(c) = insertions.get(&(template[i], template[i+1])) {
                template.insert(i+1, *c);
                i += 1;
            }

            i += 1;
        }

        println!("step {}", _step);
    }

    let mut count: HashMap<char, i32> = HashMap::new();
    for i in template {
        if let Some(c) = count.get_mut(&i) {
            *c += 1;
        } else {
            count.insert(i, 1);
        }
    }

    let mut min = i32::max_value();
    let mut max = 0;
    for (_, i) in count {
        if i > max { max = i; }
        if i < min { min = i; }
    }

    max - min
}