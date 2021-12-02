pub fn run(){    
    let input = include_str!("../data/day_02.txt").trim().lines().map(|x| x.split(" ").collect::<Vec<&str>>()).map(|x| (x[0], x[1].parse::<i32>().unwrap()));
    let mut directions: Vec<(&str, i32)> = Vec::new();
    for i in input { directions.push(i); }

    let a = part_a(&directions);
    println!("Part A result: {}", a);
    let b = part_b(&directions);
    println!("Part B result: {}", b);
}

fn part_a(directions: &Vec<(&str, i32)>) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for i in directions {
        match i.0 {
            "forward" => { x += i.1 }
            "down" => { y += i.1 }
            "up" => { y -= i.1 }
            _ => {}
        }
    }

    x * y
}

fn part_b(directions: &Vec<(&str, i32)>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for i in directions {
        match i.0 {
            "forward" => { x += i.1; y += aim * i.1 }
            "down" => { aim += i.1 }
            "up" => { aim -= i.1 }
            _ => {}
        }
    }

    x * y
}