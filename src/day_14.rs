
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

    let a = part_ab(&template, &insertions, 10);
    println!("Part 1: {}", a);
    let b = part_ab(&template, &insertions, 40);
    println!("Part 2: {}", b);
}

fn part_ab(template: &Vec<char>, insertions: &HashMap<(char, char), char>, steps: usize) -> i64 {
    let mut letters: HashMap<char, i64> = HashMap::new();
    for i in 'A'..'Z' { letters.insert(i, 0); }
    for i in template { *letters.get_mut(i).unwrap() += 1; }

    let mut combos: HashMap<(char, char), i64> = HashMap::new();
    for i in 'A'..'Z' { for j in 'A'..'Z' { combos.insert((i, j), 0); }}
    for i in 0..template.len()-1 {
        *combos.get_mut(&(template[i], template[i+1])).unwrap() += 1;
    };

    for _step in 1..steps+1 {
        let mut new_combos = combos.clone();

        for (combination, letter) in insertions {
            let amount = combos.get(&combination).unwrap();
            *new_combos.get_mut(&combination).unwrap() -= amount;
            *new_combos.get_mut(&(combination.0, *letter)).unwrap() += amount;
            *new_combos.get_mut(&(*letter, combination.1)).unwrap() += amount;
            *letters.get_mut(&letter).unwrap() += amount;
        }

        combos = new_combos;
    }

    let mut min = i64::max_value();
    let mut max = 0;
    for (_, i) in letters {
        if i > max { max = i; }
        if i > 0 && i < min { min = i; }
    }

    max - min
}