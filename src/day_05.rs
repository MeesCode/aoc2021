use std::collections::HashMap;
use regex::Regex;
use std::cmp;

pub fn run(){    
    let input = include_str!("../data/day_05.txt").trim();

    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    let mut lines: Vec<((i32, i32), (i32, i32))> = Vec::new();
    
    for cap in re.captures_iter(input) {
        lines.push(((cap[1].parse().unwrap(), cap[2].parse().unwrap()), (cap[3].parse().unwrap(), cap[4].parse().unwrap())));
    }

    // println!("{:?}", lines);

    let a = part_a(&lines);
    println!("Part A result: {}", a);
    let b = part_b(&lines);
    println!("Part B result: {}", b);
}

fn part_a(lines: &Vec<((i32, i32), (i32, i32))>) -> i32{
    let mut values: HashMap<(i32, i32), i32> = HashMap::new();

    for i in lines {
        // vertical
        if i.0.0 == i.1.0 {
            for y in cmp::min(i.0.1, i.1.1)..cmp::max(i.0.1, i.1.1)+1 {
                if let Some(v) = values.get_mut(&(i.0.0, y)) {
                    *v += *v + 1;
                } else {
                    values.insert((i.0.0, y), 1);
                }
            } 
        } 

        // horizontal
        if i.0.1 == i.1.1 {
            for x in cmp::min(i.0.0, i.1.0)..cmp::max(i.0.0, i.1.0)+1 {
                if let Some(v) = values.get_mut(&(x, i.0.1)) {
                    *v += *v + 1;
                } else {
                    values.insert((x, i.0.1), 1);
                }
            } 
        } 
    }

    values.iter().fold(0, |acc, x| acc + if *x.1 >= 2 {1} else {0})
}

fn part_b(lines: &Vec<((i32, i32), (i32, i32))>) -> i32{
    let mut values: HashMap<(i32, i32), i32> = HashMap::new();

    for i in lines {
        // vertical
        if i.0.0 == i.1.0 && i.0.1 != i.1.1  {
            for y in cmp::min(i.0.1, i.1.1)..cmp::max(i.0.1, i.1.1)+1 {
                if let Some(v) = values.get_mut(&(i.0.0, y)) {
                    *v += 1;
                } else {
                    values.insert((i.0.0, y), 1);
                }
            } 
        } 

        // horizontal
        else if i.0.1 == i.1.1 && i.0.0 != i.1.0 {
            for x in cmp::min(i.0.0, i.1.0)..cmp::max(i.0.0, i.1.0)+1 {
                if let Some(v) = values.get_mut(&(x, i.0.1)) {
                    *v += 1;
                } else {
                    values.insert((x, i.0.1), 1);
                }
            } 
        } 

        // diagonal
        else {
            let mut x;
            if i.0.0 < i.1.0 {

                if i.0.1 < i.1.1 {
                    x = i.0.0;
                    for y in i.0.1..i.1.1+1 {
                        if let Some(v) = values.get_mut(&(x, y)) {
                            *v += 1;
                        } else {
                            values.insert((x, y), 1);
                        }
                        x += 1;
                    } 
                } else {
                    x = i.1.0;
                    for y in i.1.1..i.0.1+1 {
                        if let Some(v) = values.get_mut(&(x, y)) {
                            *v += 1;
                        } else {
                            values.insert((x, y), 1);
                        }
                        x -= 1;
                    } 
                }
                
            } else {

                if i.0.1 < i.1.1 {
                    x = i.0.0;
                    for y in i.0.1..i.1.1+1 {
                        if let Some(v) = values.get_mut(&(x, y)) {
                            *v += 1;
                        } else {
                            values.insert((x, y), 1);
                        }
                        x -= 1;
                    } 
                } else {
                    x = i.1.0;
                    for y in i.1.1..i.0.1+1 {
                        if let Some(v) = values.get_mut(&(x, y)) {
                            *v += 1;
                        } else {
                            values.insert((x, y), 1);
                        }
                        x += 1;
                    } 
                }
                
            };
            
        }
    }

    values.iter().fold(0, |acc, x| acc + if *x.1 >= 2 {1} else {0})
}