
use std::mem;

enum Value {
    Pair((Box<Value>, Box<Value>)),
    Literal(i32)
}

use Value::{Pair, Literal};

pub fn main(){    
    let mut input = include_str!("../data/day_18.txt").trim().lines();
    let mut tree = parse(input.next().unwrap(), 0).0;

    for i in input {
        tree = Pair((Box::new(tree), Box::new(parse(i, 0).0)));
        reduce(&mut tree);
    }

    println!("part 1: {}", magnitude(&tree));

    let all_input: Vec<&str> = include_str!("../data/day_18.txt").trim().lines().collect();
    let mut max = 0;

    for i in &all_input {
        for j in &all_input {
            if i.eq(j) { continue }

            tree = Pair((Box::new(parse(i, 0).0), Box::new(parse(j, 0).0)));
            let val = reduce(&mut tree);
            if val > max { max = val; }
        }
    }

    println!("part 2: {}", max);
}

fn reduce(tree: &mut Value) -> i32{
    loop {
        if !explode(tree) && !split(tree) {break;}
    }
    magnitude(tree)
}

fn split(tree: &mut Value) -> bool{
    if let Pair((left, right)) = tree {
        if split(left) { return true }
        return split(right);
    }

    if let Literal(val) = tree {
        if *val >= 10 {
            let l = *val/2;
            let r = *val/2 + if *val % 2 != 0 {1} else {0};

            let _ = mem::replace(tree, Pair( (Box::new(Literal(l)), Box::new(Literal(r)))));
            return true;
        }
    }

    false
}

fn explode(tree: &mut Value) -> bool {
    let (index, found, left, right) = explode_step(tree, 0, 0);
    if !found { return false; } 
    fill(tree, 0, index, left, right);
    true
}

fn fill(tree: &mut Value, index: i32, target: i32, l: i32, r: i32) -> i32 {
    if let Pair((left, right)) = tree {
        let i = fill(left, index, target, l, r);
        let i = fill(right, i, target, l, r);
        return i;
    }

    if let Literal(val) = tree {
        if index == target - 1 {
            *val += l;
        }
        if index == target + 1 {
            *val += r;
        }
        return index+1;
    }

    panic!("unhandeled recusive return");
}

fn explode_step(tree: &mut Value, depth: i32, index: i32) -> (i32, bool, i32, i32) {
    
    if let Pair((left, right)) = tree {
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

            return (index, true, l, r);
        } else {
            let (l, foundl, le, ri) = explode_step(left, depth + 1, index);
            if foundl { return (l, true, le, ri); }
            let (r, foundr, le, ri) = explode_step(right, depth + 1, l);
            if foundr {
                return (r, true, le, ri);
            } else {
                return (r, false, le, ri);
            }
        }
    }

    if let Literal(_) = tree {
        return (index+1, false, 0, 0);
    }

    panic!("unhandeled recusive return");
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
