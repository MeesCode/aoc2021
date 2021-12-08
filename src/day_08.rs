use regex::Regex;

pub fn main(){    
    let input = include_str!("../data/day_08.txt");
    let mut entries: Vec<[String; 14]> = Vec::new();

    let re = Regex::new(r"(\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) \| (\w+) (\w+) (\w+) (\w+)").unwrap();

    for cap in re.captures_iter(input) {
        entries.push(
            [cap[1].to_string(), cap[2].to_string(), cap[3].to_string(), cap[4].to_string(), cap[5].to_string(), 
            cap[6].to_string(), cap[7].to_string(), cap[8].to_string(), cap[9].to_string(), cap[10].to_string(),
            cap[11].to_string(), cap[12].to_string(), cap[13].to_string(), cap[14].to_string()]
        );
    }

    // println!("{:?}", entries);

    let a = part_a(&entries);
    println!("Part A result: {}", a);
    let b = part_b(&entries);
    println!("Part B result: {}", b);
}

fn part_a(entries: &Vec<[String; 14]>) -> i32 {
    entries.iter().fold(0, |acc, i| acc + i.iter().enumerate().fold(0, |acc2, (index, j)| acc2 + if index > 9 && (j.len() == 2 || j.len() == 4 || j.len() == 3 || j.len() == 7) {1} else {0}) )
}

fn permutations(list: Vec<Vec<char>>, current: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if list.is_empty() { return current; }

    let mut list = list.clone();
    let entries = list.remove(0);

    if current.len() == 0 { return permutations(list, vec![entries]) }

    let mut new_current = Vec::new();

    for i in entries {

        for c in &current {

            if c.contains(&i) { continue; }

            let mut d = c.clone();
            d.push(i);
            new_current.push(d);
        }

    }

    permutations(list, new_current)
}

fn part_b(entries: &Vec<[String; 14]>) -> i32 {

    let mut total_res = 0;

    let zero: Vec<i32>   = vec![0,1,2,4,5,6];
    let one: Vec<i32>   = vec![2,5];
    let two: Vec<i32>   = vec![0,2,3,4,6];
    let three: Vec<i32> = vec![0,2,3,5,6];
    let four: Vec<i32>  = vec![1,2,3,5];
    let five: Vec<i32>  = vec![0,1,3,5,6];
    let six: Vec<i32>   = vec![0,1,3,4,5,6];
    let seven: Vec<i32> = vec![0,2,5];
    let eight: Vec<i32> = vec![0,1,2,3,4,5,6];
    let nine: Vec<i32>  = vec![0,1,2,3,5,6];

    let letters = [zero, one, two, three, four, five, six, seven, eight, nine];
    
    for entry in entries {
        let mut options = vec![vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']; 7];

        for i in entry {
            if i.len() == 2 {
                for index in [2,5] {
                    options[index] = options[index].iter().filter(|x| i.chars().collect::<Vec<char>>().contains(x)).map(|x| *x).collect();
                }
                for index in [0,1,3,4,6] {
                    options[index] = options[index].iter().filter(|x| !i.chars().collect::<Vec<char>>().contains(x)).map(|x| *x).collect();
                }
            }

            if i.len() == 3 {
                for index in [0,2,5] {
                    options[index] = options[index].iter().filter(|x| i.chars().collect::<Vec<char>>().contains(x)).map(|x| *x).collect();
                }
                for index in [1,3,4,6] {
                    options[index] = options[index].iter().filter(|x| !i.chars().collect::<Vec<char>>().contains(x)).map(|x| *x).collect();
                }
            }

            if i.len() == 4 {
                for index in [1,2,3,5] {
                    options[index] = options[index].iter().filter(|x| i.chars().collect::<Vec<char>>().contains(x)).map(|x| *x).collect();
                }
                for index in [0,4,6] {
                    options[index] = options[index].iter().filter(|x| !i.chars().collect::<Vec<char>>().contains(x)).map(|x| *x).collect();
                }
            }
        }

        let perms = permutations(options, vec![]);
        let mut found_perm = vec![];

        'outer: for p in perms {

            for i in entry {
                let mut positions = vec![];

                for c in i.chars() {
                    positions.push(p.iter().position(|x| *x == c).unwrap() as i32);
                }
                positions.sort();

                if let None = letters.iter().find(|x| positions.eq(*x)) {
                    continue 'outer;
                }
            }
            found_perm = p.clone();
        }

        let mut res = 0;

        let mut mul = 1000;
        for i in [10, 11, 12, 13].iter(){
            let mut positions = vec![];

            for c in entry[*i as usize].chars() {
                positions.push(found_perm.iter().position(|x| *x == c).unwrap() as i32);
            }
            positions.sort();

            if let Some(pos) = letters.iter().position(|x| positions.eq(x)) {
                res += pos as i32 * mul;
            }

            mul /= 10;

        }

        total_res += res;

    }

    total_res
}