
pub fn main(){    
    let mut input = include_str!("../data/day_21.txt").trim().lines();
    
    let p1 = input.next().unwrap().split(": ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()-1;
    let p2 = input.next().unwrap().split(": ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()-1;

    let a = part_a(p1, p2);
    println!("Part 1: {}", a);

    let (w1,w2) = step(p1 as u64, 0, p2 as u64, 0);
    println!("Part 1: {}", std::cmp::max(w1, w2));

}

fn part_a(p1: i32, p2: i32) -> i32 {
    let mut roll = 0;
    let mut total_rolls = 0;
    let mut p = [p1, p2];
    let mut score = [0, 0];

    let mut index = 0;

    loop {
        p[index] = (p[index] + (roll % 100 + 1) + ((roll + 1) % 100 + 1) + ((roll + 2) % 100 + 1)) % 10;
        score[index] += p[index] + 1;
        total_rolls += 3;
        roll = (roll + 3) % 100;

        if score[index] >= 1000 {
            return score[(index + 1) % 2] * total_rolls;
        }

        index = (index+1) % 2;
    }
}

fn step(p1: u64, score1: u64, p2: u64, score2: u64) -> (u64, u64) {    
    let mut wins1 = 0;
    let mut wins2 = 0;

    for i in 3..10 {
        let mul = match i { 3 => 1, 4 => 3, 5 => 6, 6 => 7, 7 => 6, 8 => 3, 9 => 1, _ => 0};

        let np1 = (p1 + i) % 10;
        let nscore1 = score1 + np1 + 1;
        if nscore1 >= 21 { wins1 += mul; continue; }

        let (w2,w1) = step(p2, score2, np1, nscore1);

        wins1 += w1 * mul;
        wins2 += w2 * mul;
    }

    return (wins1, wins2)
}