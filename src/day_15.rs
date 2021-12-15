
use priority_queue::PriorityQueue;

pub fn main(){    
    let grid: Vec<Vec<i32>> = include_str!("../data/day_15.txt").trim().lines().map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as i32).collect()).collect();
    
    let a = part_ab(&grid);
    println!("Part 1: {}", a);

    let mut grid2: Vec<Vec<i32>> = vec![vec![0; grid[0].len()*5]; grid.len()*5];
    for y in 0..grid.len()*5 {
        for x in 0..grid[0].len()*5 {
            grid2[y][x] = (((grid[y % grid.len()][x % grid[0].len()] -1) + (y / grid.len()) as i32 + (x / grid[0].len()) as i32) % 9) + 1;
        }
    }

    let b = part_ab(&grid2);
    println!("Part 2: {}", b);
}

fn part_ab(grid: &Vec<Vec<i32>>) -> i32 {

    let mut distances = vec![vec![i32::max_value(); grid[0].len()]; grid.len()];
    distances[0][0] = 0;

    let mut queue = PriorityQueue::new();
    queue.push((0, 0), i32::max_value());

    while !queue.is_empty() {

        let ((x, y), _) = queue.pop().unwrap();

        if x == grid[0].len()-1 && y == grid.len()-1 { break; }

        let mut n: Vec<(usize, usize)> = Vec::with_capacity(4);
        if x > 0 { n.push((x-1, y)); }
        if x < grid[0].len()-1 { n.push((x+1, y)); }
        if y > 0 { n.push((x, y-1)); }
        if y < grid[0].len()-1 { n.push((x, y+1)); }

        for (dx, dy) in n {
            if distances[y][x] + grid[dy][dx] < distances[dy][dx] {
                queue.push((dx, dy), i32::max_value() - (distances[y][x] + grid[dy][dx]));
                distances[dy][dx] = distances[y][x] + grid[dy][dx];
            }
        }

    }

    distances[grid.len()-1][grid[0].len()-1]
}