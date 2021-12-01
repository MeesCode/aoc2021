pub fn run(){
    let input: String = String::from(include_str!("../data/day_01.txt").trim());

    let mut nums: Vec<i32> = Vec::new();
    for i in input.lines(){
        nums.push(i.parse::<i32>().unwrap());
    }
    
    let a = part_a(&nums);
    println!("Part A result: {}", a);
    let b = part_b(&nums);
    println!("Part B result: {}", b);
}

fn part_a(nums: &Vec<i32>) -> i32 {
    let mut iter = nums.iter();
    let mut counter = 0;
    let mut test = iter.next().unwrap();
    for i in iter {
        if i > test {
            counter += 1;
        }
        test = i;
    }
    counter
}

fn part_b(nums: &Vec<i32>) -> i32 {
    let mut counter = 0;
    for i in 0..nums.len()-3 {
        if nums[i] + nums[i+1] + nums[i+2] < nums[i+1] + nums[i+2] + nums[i+3] {
            counter += 1;
        }
    }
    counter
}

