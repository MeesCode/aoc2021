use std::collections::HashSet;

pub fn main(){    
    let input = include_str!("../data/day_09.txt").trim().lines();
    
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for i in input {
        grid.push( i.chars().map(|x| x.to_digit(10).unwrap() as i32).collect() )
    }

    let a = part_a(&grid);
    println!("Part A result: {}", a);
    let b = part_b(&grid);
    println!("Part B result: {}", b);
}

fn get_lowest(grid: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut lows = Vec::new();
    for y in 0..grid.len() {
        'outer: for x in 0..grid[0].len() {
            for (dx, dy) in [(x, y-1), (x-1, y), (x+1, y), (x, y+1), (x-1, y-1), (x-1, y+1), (x+1, y-1), (x+1, y+1)] {
                if dx >= grid[0].len() || dy >= grid.len() { continue; }
                if grid[dy][dx] < grid[y][x] { continue 'outer; } 
            }
            lows.push((x, y));
        }
    }
    lows
}

fn part_a(grid: &Vec<Vec<i32>>) -> i32 {
    get_lowest(grid).iter().fold(0, |a, &(x, y)| a + grid[y][x] + 1)
}

fn part_b(grid: &Vec<Vec<i32>>) -> i32 {
    let mut sizes = Vec::new();
    for (x, y) in get_lowest(grid) {
        let mut visited = HashSet::new();
        visited.insert((x,y));
        let mut current: Vec<(usize, usize)> = vec![(x,y)];

        while !current.is_empty() {
            let mut new_current = Vec::new();
            for (cx, cy) in current {
                for (dx, dy) in [(cx, cy-1), (cx-1, cy), (cx+1, cy), (cx, cy+1)] {
                    if dx >= grid[0].len() || dy >= grid.len() { continue; }
                    if grid[dy][dx] != 9 && !visited.contains(&(dx, dy)) {
                        new_current.push((dx, dy));
                        visited.insert((dx, dy));
                    }
                }
            }
            current = new_current.clone();
        }
        sizes.push(visited.len() as i32);
    }

    sizes.sort();
    sizes.pop().unwrap() * sizes.pop().unwrap() * sizes.pop().unwrap()
}
