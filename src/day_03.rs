pub fn run(){    
    let input: Vec<&str> = include_str!("../data/day_03.txt").trim().lines().collect();

    let a = part_a(&input);
    println!("Part A result: {}", a);
    let b = part_b(&input);
    println!("Part B result: {}", b);
}

fn bin2int(input: &Vec<i32>) -> i32 {
    let mut rev = input.clone();
    rev.reverse();
    rev.iter().fold((0, 0), |acc, x| (acc.0 + if *x == 1 { 1 << acc.1 } else { 0 }, acc.1+1)).0
}

fn count_pos(input: &Vec<&str>, pos: usize) -> i32 {
    input.iter().fold(0, |acc, x| acc + if x.chars().nth(pos).unwrap() == '1' { 1 } else { 0 })
} 

fn part_a(input: &Vec<&str>) -> i32 {
    let count: Vec<i32> = vec![0; input[0].len()].iter().enumerate().map(|(i, _)| count_pos(input, i)).collect();
    let binary: Vec<i32> = count.iter().map(|x| if *x > input.len() as i32 / 2 { 1 } else { 0 }).collect();
    let gamma = bin2int(&binary);
    let epsilon = bin2int(&binary.iter().map(|x| if *x == 1 { 0 } else { 1 }).collect::<Vec<i32>>());
    gamma * epsilon
}

fn part_b(input: &Vec<&str>) -> i32 {
    let mut selection: Vec<&str> = input.clone();

    let mut index = 0;
    while selection.len() > 1 {
        let c = count_pos(&selection, index);
        let keep = if c >= selection.len() as i32 - c { '1' } else { '0' };
        selection = selection.iter().filter(|x| x.chars().nth(index).unwrap() == keep).map(|x| *x).collect();
        index += 1;
    }

    let oxy = bin2int(&selection[0].chars().map(|x| x.to_digit(10).unwrap() as i32).collect());

    selection = input.clone();

    let mut index = 0;
    while selection.len() > 1 {
        let c = count_pos(&selection, index);
        let keep = if c < selection.len() as i32 - c { '1' } else { '0' };
        selection = selection.iter().filter(|x| x.chars().nth(index).unwrap() == keep).map(|x| *x).collect();
        index += 1;
    }

    let scrub = bin2int(&selection[0].chars().map(|x| x.to_digit(10).unwrap() as i32).collect());
    
    oxy * scrub
}