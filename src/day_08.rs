pub fn main(){    
    let input = include_str!("../data/day_08.txt").lines();
    let mut entries: Vec<(Vec<&str>, Vec<&str>)> = Vec::new();

    for i in input {
        let parts: Vec<&str> = i.split(" | ").collect();
        entries.push((
            parts[0].split(" ").collect(),
            parts[1].split(" ").collect(),
        ));
    }
    
    let a = part_a(&entries);
    println!("Part A result: {}", a);
    let b = part_b(&entries);
    println!("Part B result: {}", b);
}

fn part_a(entries: &Vec<(Vec<&str>, Vec<&str>)>) -> i32 {
    entries.iter().fold(0, |acc, i| acc + i.1.iter().fold(0, |acc2, j| acc2 + if j.len() == 2 || j.len() == 4 || j.len() == 3 || j.len() == 7 {1} else {0}) )
}

fn part_b(entries: &Vec<(Vec<&str>, Vec<&str>)>) -> i32 {
    let mut result = 0;

    for entry in entries {
        let mut one = Vec::new();
        let mut four = Vec::new();
        let mut seven = Vec::new();

        for i in &entry.0 {
            if i.len() == 2 { one = i.chars().collect(); } 
            if i.len() == 4 { four = i.chars().collect(); } 
            if i.len() == 3 { seven = i.chars().collect(); } 
        }

        let mut mul = 1000;
        for i in &entry.1 {
            let overlap_one = i.chars().filter(|x| one.contains(x)).count();
            let overlap_four = i.chars().filter(|x| four.contains(x)).count();
            let overlap_seven = i.chars().filter(|x| seven.contains(x)).count();

            let number;
            if i.len() == 6 && overlap_four == 3 && overlap_seven == 3 { number = 0; }
            else if i.len() == 2 { number = 1; }
            else if i.len() == 5 && overlap_seven == 2 && overlap_four == 2 { number = 2; }
            else if i.len() == 5 && overlap_seven == 3 { number = 3; }
            else if i.len() == 4 { number = 4; }
            else if i.len() == 5 && overlap_seven == 2 && overlap_four == 3 { number = 5; }
            else if i.len() == 6 && overlap_one == 1 { number = 6; }
            else if i.len() == 3 { number = 7; }
            else if i.len() == 7 { number = 8; }
            else { number = 9 };

            result += mul * number;
            mul /= 10;
        }
    }

    result
}