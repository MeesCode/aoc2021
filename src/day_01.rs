pub fn run(){    
    let a = include_str!("../data/day_01.txt").trim().lines().map(|x| x.parse::<i32>().unwrap()).fold((10000, 10000, 10000, 0, 0), |acc, x| (acc.1, acc.2, x, acc.3 + if x > acc.2 { 1 } else { 0 }, acc.4 + if x + acc.1 + acc.2 > acc.0 + acc.1 + acc.2 { 1 } else { 0 }));
    println!("Part A result: {}", a.3);
    println!("Part B result: {}", a.4);
}