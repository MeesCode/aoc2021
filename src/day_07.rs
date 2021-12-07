pub fn main(){
    let input: Vec<i64> = include_str!("../data/day_07.txt").trim().split(",").map(|x| x.parse().unwrap()).collect();

    let a = part_a(&input);
    println!("Part A result: {}", a);
    let b = part_b(&input);
    println!("Part B result: {}", b);
}

fn part_a(input: &Vec<i64>) -> i64 {
    let max: i64 = *input.iter().max().unwrap();
    let mut min = 999999999;
    for i in 0..max+1 {
        let val = input.iter().fold(0, |acc, x| acc + i64::abs(x - i));
        if val < min { min = val; }
    }
    min
}

fn part_b(input: &Vec<i64>) -> i64 {
    let max: i64 = *input.iter().max().unwrap();
    let mut min: i64 = 99999999999999999;
    for i in 0..max+1 {
        let val: i64 = input.iter().fold(0, |acc, x| acc + ((i64::abs(x - i)) * ((i64::abs(x - i)) + 1))/2);
        if val < min { min = val; }
    }
    min
}