pub fn main(){    
    let input = include_str!("../data/day_02.txt").trim().lines().map(|x| x.split(" ").collect::<Vec<&str>>()).map(|x| (x[0], x[1].parse::<i32>().unwrap()));
    let mut directions: Vec<(&str, i32)> = Vec::new();
    for i in input { directions.push(i); }

    let a = part_a(&directions);
    println!("Part A result: {}", a);
    let b = part_b(&directions);
    println!("Part B result: {}", b);
}

fn part_a(directions: &Vec<(&str, i32)>) -> i32 {
    let res = directions.iter().fold((0, 0), |acc, x| {
        match x.0 {
            "forward" => (acc.0 + x.1, acc.1),
            "down"    => (acc.0, acc.1 + x.1),
            "up"      => (acc.0, acc.1 - x.1),
            _         => (0, 0)
        }
    });
    res.0 * res.1
}

fn part_b(directions: &Vec<(&str, i32)>) -> i32 {
    let res = directions.iter().fold((0, 0, 0), |acc, x| {
        match x.0 {
            "forward" => (acc.0 + x.1, acc.1 + acc.2 * x.1, acc.2),
            "down"    => (acc.0, acc.1, acc.2 + x.1),
            "up"      => (acc.0, acc.1, acc.2 - x.1),
            _         => (0, 0, 0)
        }
    });
    res.0 * res.1
}