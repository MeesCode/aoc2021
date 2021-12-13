pub fn main(){
    let input: Vec<u8> = include_str!("../data/day_06.txt").trim().split(",").map(|x| x.parse().unwrap()).collect();
    let a = part_ab(&input, 80);
    println!("Part 1: {}", a);
    let b = part_ab(&input, 256);
    println!("Part 2: {}", b);
}

fn part_ab(input: &Vec<u8>, rounds: i32) -> u64 {
    let mut nums = [0 as u64; 9];
    for i in input { nums[*i as usize] += 1; }
    for index in 0..rounds {
        nums[(index as usize + 7) % 9] += nums[index as usize % 9] as u64;
    }
    nums.iter().fold(0, |acc, x| acc + x)
}
