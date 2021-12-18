
use regex::Regex;
use std::cmp;

pub fn main(){    
    let input = include_str!("../data/day_17.txt").trim();

    let re = Regex::new(r"target area: x=(\d+)..(\d+), y=-(\d+)..-(\d+)").unwrap();
    let cap = re.captures(input).unwrap();
    let target_x: (i32, i32) = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
    let target_y: (i32, i32) = (cap[3].parse::<i32>().unwrap() * -1, cap[4].parse::<i32>().unwrap() * -1);
    
    let (a, b) = part_ab(target_x, target_y);
    println!("Part 1: {}", a);
    println!("Part 2: {}", b);
}

fn part_ab(target_x: (i32, i32), target_y: (i32, i32)) -> (i32, i32) {
    let mut result = 0;
    let mut count = 0;

    for sdx in 0..target_x.1+1 {
        for sdy in target_y.0..1000 {
            let mut dx = sdx;
            let mut dy = sdy;

            let mut x = 0;
            let mut y = 0;
            let mut max_y = 0;

            while x <= target_x.1 && y >= target_y.0{
                if x >= target_x.0 && y <= target_y.1 { 
                    result = cmp::max(result, max_y);
                    count += 1;
                    break;
                }

                x += dx;
                y += dy;

                if dx > 0 { dx -= 1 };
                dy -= 1;

                max_y = cmp::max(y, max_y);
            }        
        }
    }

    (result, count)
}