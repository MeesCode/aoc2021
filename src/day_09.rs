use std::collections::HashSet;

pub fn main(){    
    let input = include_str!("../data/day_09.txt").trim().lines();
    
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for i in input {
        grid.push(
            i.chars().map(|x| x.to_digit(10).unwrap() as i32).collect()
        )
    }

    let a = part_a(&grid);
    println!("Part A result: {}", a);
    let b = part_b(&grid);
    println!("Part B result: {}", b);
}

fn part_a(grid: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for y in 0 as i32..grid.len() as i32 {
        'outer: for x in 0 as i32..grid[0].len() as i32 {

            for dy in y as i32-1..y as i32+2 {
                for dx in x as i32-1..x as i32+2 {
                    if dx == x && dy == y { continue; }
                    if dx == -1 || dx >= grid[0].len() as i32 || dy == -1 || dy >= grid.len() as i32 { continue; }
                    if grid[dy as usize][dx as usize] < grid[y as usize][x as usize] { continue 'outer; } 
                }
            }

            result += grid[y as usize][x as usize] + 1;
        }
    }

    result
}

fn part_b(grid: &Vec<Vec<i32>>) -> i32 {
    let mut result = 1;
    let mut lows = Vec::new();
    let mut sizes = Vec::new();

    for y in 0 as i32..grid.len() as i32 {
        'outer: for x in 0 as i32..grid[0].len() as i32 {

            for dy in y as i32-1..y as i32+2 {
                for dx in x as i32-1..x as i32+2 {
                    if dx == x && dy == y { continue; }
                    if dx == -1 || dx >= grid[0].len() as i32 || dy == -1 || dy >= grid.len() as i32 { continue; }
                    if grid[dy as usize][dx as usize] < grid[y as usize][x as usize] { continue 'outer; } 
                }
            }

            lows.push((x, y));
        }
    }

    for (x, y) in lows {

        let mut visited = HashSet::new();
        visited.insert((x,y));
        let mut current = vec![(x,y)];

        while !current.is_empty() {

            let mut new_current = Vec::new();

            for (cx, cy) in &current {

                for dy in *cy as i32-1..*cy as i32+2 {
                    for dx in *cx as i32-1..*cx as i32+2 {

                        if dx == *cx && dy == *cy { continue; }
                        if dx == -1 || dx >= grid[0].len() as i32 || dy == -1 || dy >= grid.len() as i32 { continue; }
                        if dx != *cx && dy != *cy { continue; }

                        // println!("({}, {})", dx, dy);

                        if grid[dy as usize][dx as usize] != 9 && !visited.contains(&(dx, dy)) {
                            new_current.push((dx, dy));
                            visited.insert((dx, dy));
                        }
                    }
                }

            }

            current = new_current.clone();
        }

        sizes.push(visited.len() as i32);

    }

    sizes.sort();

    for i in sizes.len()-3..sizes.len() {
        result *= sizes[i];
    }

    result
}

