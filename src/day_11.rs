pub fn main(){    
    let grid: Vec<Vec<i32>> = include_str!("../data/day_11.txt").trim().lines().map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as i32).collect()).collect();
    
    let (a, b) = part_ab(&grid);
    println!("Part A result: {}", a);
    println!("Part B result: {}", b);
}

fn part_ab(grid: &Vec<Vec<i32>>) -> (i32, i32) {
    let mut grid = grid.clone();
    let mut flashes = 0;
    let mut a = 0;
    let mut step = 1;

    loop {
        grid = grid.iter().map(|y| y.iter().map(|x| *x + 1).collect()).collect();

        let mut done = false;
        let mut cur_flashes = 0;
        while !done { 
            done = true;
            for y in 0..10 {
                for x in 0..grid[y].len() {
                    if grid[y][x] > 9 {
                        grid[y][x] = -1;
                        flashes += 1;
                        cur_flashes += 1;
                        done = false;
                        for (dx, dy) in [(x-1, y-1), (x, y-1), (x+1, y-1), (x-1, y), (x+1, y), (x-1, y+1), (x, y+1), (x+1, y+1)] {
                            if dx >= 10 || dy >= 10 || grid[dy][dx] == -1 { continue; }
                            grid[dy][dx] += 1;
                        }
                    }
                }
            }
        }

        if step == 100 { a = flashes; }
        if cur_flashes == 100 { break; }

        grid = grid.iter().map(|y| y.iter().map(|x| if *x == -1 {0} else {*x}).collect()).collect();

        step += 1;
    }

    (a, step)
}