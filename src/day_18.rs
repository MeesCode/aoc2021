
use std::mem;
use itertools::Itertools;
use std::cmp;

#[derive(Clone)]
enum Value {
    Pair((Box<Value>, Box<Value>)),
    Literal(i32)
}

use Value::{Pair, Literal};

pub fn main(){    
    let input: Vec<Value> = include_str!("../data/day_18.txt").trim().lines().map(|x| parse(x, 0).0).collect();
    
    let mut tree = input[0].clone();
    for i in 1..input.len() {
        tree = reduce(&Pair((Box::new(tree), Box::new(input[i].clone()))));
    }
    println!("part 1: {}", magnitude(&tree));

    let b = (0..input.len()).permutations(2).fold(0, |a, x| cmp::max(a, if x[0] == x[1] {0} else { magnitude(&reduce(&Pair((Box::new(input[x[0]].clone()), Box::new(input[x[1]].clone()))))) } ));
    println!("part 2: {}", b);
}

fn reduce(tree: &Value) -> Value {
    let mut tree = tree.clone();
    loop {
        if !explode(&mut tree) && !split(&mut tree) { break; }
    }
    tree
}

fn split(tree: &mut Value) -> bool{
    match tree {
        Pair((left, right)) => {
            if split(left) { return true; }
            split(right)
        },
        Literal(val) => {
            if *val >= 10 {
                let l = *val/2;
                let r = *val/2 + if *val % 2 != 0 {1} else {0};
                let _ = mem::replace(tree, Pair((Box::new(Literal(l)), Box::new(Literal(r)))));
                return true;
            }
            false
        }
    }
}

fn explode(tree: &mut Value) -> bool {
    let (index, found, left, right) = explode_step(tree, 0, 0);
    if !found { return false; } 
    fill(tree, 0, index, left, right);
    true
}

fn fill(tree: &mut Value, index: i32, target: i32, l: i32, r: i32) -> i32 {
    match tree {
        Pair((left, right)) => {
            let i = fill(left, index, target, l, r);
            fill(right, i, target, l, r)
        },
        Literal(val) => {
            if index == target - 1 {
                *val += l;
            }
            if index == target + 1 {
                *val += r;
            }
            index+1
        }
    }
}

fn explode_step(tree: &mut Value, depth: i32, index: i32) -> (i32, bool, i32, i32) {
    match tree {
        Pair((left, right)) => {
            if depth == 4 {

                let l = match **left {
                    Literal(x) => x,
                    _ => 0
                };
                let r = match **right {
                    Literal(x) => x,
                    _ => 0
                };

                let _ = mem::replace(tree, Literal(0));

                (index, true, l, r)
            } else {
                let (l, foundl, le, ri) = explode_step(left, depth + 1, index);
                if foundl { return (l, true, le, ri); }
                let (r, foundr, le, ri) = explode_step(right, depth + 1, l);
                if foundr {
                    (r, true, le, ri)
                } else {
                    (r, false, le, ri)
                }
            }
        },
        Literal(_) => {
            (index+1, false, 0, 0)
        }
    }
}

fn magnitude(tree: &Value) -> i32 {
    match tree {
        Pair((left, right)) => 3 * magnitude(left) + 2 * magnitude(right),
        Literal(lit) => *lit
    }
}

fn _format(input: &Value) -> String {
    match input {
        Pair((left, right)) => format!("[{},{}]", _format(left), _format(right)),
        Literal(lit) => lit.to_string()
    }
}

fn parse(input: &str, index: usize) -> (Value, usize) {
    if let Some(c) = input.chars().nth(index) {
        if c == ',' || c == ']' {
            return parse(input, index + 1)
        }

        if c == '[' {
            let (left, index) = parse(input, index+1);
            let (right, index) = parse(input, index);
            return (Pair((Box::new(left), Box::new(right))), index + 1)
        } else {
            return (Literal(c.to_digit(10).unwrap() as i32), index + 1)
        }
    }
   
    panic!("could not parse day 18 input")
}
