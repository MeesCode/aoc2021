
use std::collections::HashSet;

pub fn main(){    
    let input = include_str!("../data/day_13.txt").trim().lines();
    let mut dots: Vec<(i32, i32)> = Vec::new();
    let mut instructions: Vec<(char, i32)> = Vec::new();

    let mut cords_done = false;
    for i in input {
        if i == "" { cords_done = true; continue; }

        if !cords_done {
            let parts: Vec<i32> = i.split(",").map(|x| x.parse().unwrap()).collect();
            dots.push((parts[0], parts[1]));
        } else {
            let parts: Vec<&str> = i.split("=").collect();
            instructions.push((parts[0].chars().nth(11).unwrap(), parts[1].parse().unwrap()));
        }
    }

    let a = part_a(&dots, &instructions);
    println!("Part 1: {}", a);
    println!("Part 2:");
    part_b(&dots, &instructions);
}

fn fold(dots: &mut Vec<(i32, i32)>, instruction: (char, i32)) {
    if instruction.0 == 'y' {
        for d in dots.iter_mut() {
            if d.1 > instruction.1 {
                d.1 = d.1 - (d.1 - instruction.1)*2;
            }
        }
    }

    if instruction.0 == 'x' {
        for d in dots.iter_mut() {
            if d.0 > instruction.1 {
                d.0 = d.0 - (d.0 - instruction.1)*2;
            }
        }
    }
}

fn part_a(dots: &Vec<(i32, i32)>, instructions: &Vec<(char, i32)>) -> i32 {
    let mut dots = dots.clone();
    fold(&mut dots, instructions[0]);
    let dedup: HashSet<(i32, i32)> = HashSet::from_iter(dots);
    dedup.len() as i32
}

fn part_b(dots: &Vec<(i32, i32)>, instructions: &Vec<(char, i32)>) {
    let mut dots = dots.clone();

    for instruction in instructions {
        fold(&mut dots, *instruction);
    }

    let (min_x, min_y) = instructions.iter().fold((0, 0), |acc, x| if x.0 == 'x' { (x.1, acc.1) } else { (acc.0, x.1) });
    let mut grid = vec![vec![' '; min_x as usize]; min_y as usize];

    for i in dots {
        grid[i.1 as usize][i.0 as usize] = '#';
    }

    for y in grid {
        for x in y {
            print!("{}", x);
        }
        println!();
    }

}