pub fn main(){    
    let input = include_str!("../data/day_08.txt").lines();
    let mut entries: Vec<(Vec<&str>, Vec<&str>)> = Vec::new();

    for i in input {
        let parts: Vec<&str> = i.split(" | ").collect();
        entries.push( (parts[0].split(" ").collect(), parts[1].split(" ").collect()) );
    }
    
    let a = part_a(&entries);
    println!("Part 1: {}", a);
    let b = part_b(&entries);
    println!("Part 2: {}", b);
}

fn part_a(entries: &Vec<(Vec<&str>, Vec<&str>)>) -> i32 {
    entries.iter().fold(0, |a, i| a + i.1.iter().fold(0, |a, i| a + if [2,3,4,7].contains(&i.len()) {1} else {0}) )
}

fn part_b(entries: &Vec<(Vec<&str>, Vec<&str>)>) -> i32 {
    let mut result = 0;

    for entry in entries {
        let one:   Vec<char> = entry.0.iter().find(|x| x.len() == 2).unwrap().chars().collect();
        let four:  Vec<char> = entry.0.iter().find(|x| x.len() == 4).unwrap().chars().collect();
        let seven: Vec<char> = entry.0.iter().find(|x| x.len() == 3).unwrap().chars().collect();

        for (index, i) in entry.1.iter().enumerate() {
            let overlap_one = i.chars().filter(|x| one.contains(x)).count();
            let overlap_four = i.chars().filter(|x| four.contains(x)).count();
            let overlap_seven = i.chars().filter(|x| seven.contains(x)).count();

            let number =
                if i.len() == 6 && overlap_four == 3 && overlap_seven == 3 { 0 }
                else if i.len() == 2 { 1 }
                else if i.len() == 5 && overlap_seven == 2 && overlap_four == 2 { 2 }
                else if i.len() == 5 && overlap_seven == 3 { 3 }
                else if i.len() == 4 { 4 }
                else if i.len() == 5 && overlap_seven == 2 && overlap_four == 3 { 5 }
                else if i.len() == 6 && overlap_one == 1 { 6 }
                else if i.len() == 3 { 7 }
                else if i.len() == 7 { 8 }
                else { 9 };

            result += 1000 / i32::pow(10, index as u32) * number;
        }
    }

    result
}