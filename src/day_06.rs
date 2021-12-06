pub fn run(){
    let input: Vec<u8> = include_str!("../data/day_06.txt").trim().split(",").map(|x| x.parse().unwrap()).collect();

    let a = part_ab(&input, 80);
    println!("Part A result: {}", a);
    let b = part_ab(&input, 256);
    println!("Part B result: {}", b);
}

fn part_ab(input: &Vec<u8>, rounds: i32) -> u64 {
    let mut nums = [0 as u64; 9];
    for i in input { nums[*i as usize] += 1; }
    for _ in 0..rounds {
        let mut cur_nums = [0 as u64; 9];
        cur_nums[6] += nums[0] as u64;
        cur_nums[8] += nums[0] as u64;
        for i in 0..8 {
            cur_nums[i] += nums[i+1];
        }
        nums = cur_nums;
    }

    nums.iter().fold(0, |acc, x| acc + x)
}
