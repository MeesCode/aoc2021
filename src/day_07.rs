pub fn main(){
    let input: Vec<i64> = include_str!("../data/day_07.txt").trim().split(",").map(|x| x.parse().unwrap()).collect();

    let a = part_a(&input);
    println!("Part 1: {}", a);
    let b = part_b(&input);
    println!("Part 2: {}", b);
}

fn part_a(input: &Vec<i64>) -> i64 {
    let mut input = input.clone();
    input.sort();
    let median = input[input.len()/2];
    input.iter().fold(0, |acc, x| acc + i64::abs(x - median))
}

fn part_b(input: &Vec<i64>) -> i64 {
    let max = *input.iter().max().unwrap();
    let mut min = i64::max_value();
    for i in 0..max+1 {
        let val = input.iter().fold(0, |acc, x| acc + ((i64::abs(x - i)) * ((i64::abs(x - i)) + 1))/2);
        min = std::cmp::min(val, min);
    }
    min
}