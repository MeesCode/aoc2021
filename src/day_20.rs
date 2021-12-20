
use std::collections::HashSet;
use std::cmp::{min, max};

pub fn main(){    
    let mut input = include_str!("../data/day_20.txt").trim().lines();
    let algorithm: Vec<char> = input.next().unwrap().chars().collect();
    input.next();
    
    let mut pixels: HashSet<(i32, i32)> = HashSet::new();

    for (index_y, y) in input.enumerate() {
        for (index_x, x) in y.chars().enumerate() {
            if x == '#' {
                pixels.insert((index_x as i32, index_y as i32));
            }
        }
    }
    
    let a = part_ab(&pixels, &algorithm, 2);
    println!("Part 1: {}", a);
    let b = part_ab(&pixels, &algorithm, 50);
    println!("Part 2: {}", b);
}


fn part_ab(pixels: &HashSet<(i32, i32)>, algorithm: &Vec<char>, rounds: i32) -> i32 {
    let mut pixels = pixels.clone();
    let mut pixels_buff;

    let (max_x, max_y) = pixels.iter().fold((0, 0), |a,x| (max(a.0, x.0), max(a.1, x.1)));

    for _ in 0..rounds {
        pixels_buff = pixels.clone();

        for y in 0-(rounds*2)..max_y+(rounds*2) {
            for x in 0-(rounds*2)..max_y+(rounds*2) {

                let mut bin = 0;
                let mut pow = 8;
                for dy in y-1..y+2 {
                    for dx in x-1..x+2 {
                        
                        if pixels.contains(&(dx,dy)) {
                            bin += i32::pow(2, pow);
                        }

                        pow -= 1;
                    }
                }

                if algorithm[bin as usize] == '#' {
                    pixels_buff.insert((x,y));
                } else {
                    pixels_buff.remove(&(x,y));
                }

            }
        }

        pixels = pixels_buff.clone();
    }
    
    pixels.iter().filter(|(x, y)| *x >= 0-rounds && *x <= max_x+rounds && *y >= 0-rounds && *y <= max_y+rounds).count() as i32
}

fn _format(pixels: &HashSet<(i32, i32)>) -> String {
    let (min_x, max_x, min_y, max_y) = pixels.iter().fold((i32::MAX, i32::MIN, i32::MAX, i32::MIN), |a,x| (min(a.0, x.0), max(a.1, x.0), min(a.2, x.1), max(a.3, x.1)));
    let mut res = String::new();
    for y in min_y..max_y+1 {
        for x in min_x..max_x+1 {
            res.push(if let Some(_) = pixels.get(&(x, y)) { '#' } else { '.' });
        }
        if y != max_y { res.push('\n') };
    }
    res
}