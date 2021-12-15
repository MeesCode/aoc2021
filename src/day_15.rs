
use priority_queue::PriorityQueue;

pub fn main(){    
    let grid: Vec<Vec<i32>> = include_str!("../data/day_15.txt").trim().lines().map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as i32).collect()).collect();
    
    let a = part_ab(&grid, 1);
    println!("Part 1: {}", a);
    let b = part_ab(&grid, 5);
    println!("Part 2: {}", b);
}

fn part_ab(grid: &Vec<Vec<i32>>, multiplier: usize) -> i32 {
    let mut distances = vec![vec![i32::MAX; grid[0].len()*multiplier]; grid.len()*multiplier];
    distances[0][0] = 0;

    let mut queue = PriorityQueue::new();
    queue.push((0, 0), i32::MAX);

    while let Some(((x, y), _)) = queue.pop() {
        if x == distances[0].len()-1 && y == distances.len()-1 { break; }

        let mut neighbours: Vec<(usize, usize)> = Vec::with_capacity(4);
        if x > 0 { neighbours.push((x-1, y)); }
        if x < distances[0].len()-1 { neighbours.push((x+1, y)); }
        if y > 0 { neighbours.push((x, y-1)); }
        if y < distances[0].len()-1 { neighbours.push((x, y+1)); }

        for (dx, dy) in neighbours {
            let dist = (((grid[dy % grid.len()][dx % grid[0].len()] - 1) + (dy / grid.len()) as i32 + (dx / grid[0].len()) as i32) % 9) + 1;
            if distances[y][x] + dist < distances[dy][dx] {
                queue.push((dx, dy), i32::MAX - (distances[y][x] + dist));
                distances[dy][dx] = distances[y][x] + dist;
            }
        }
    }

    *distances.last().unwrap().last().unwrap()
}