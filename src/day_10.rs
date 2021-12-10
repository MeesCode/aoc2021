pub fn main(){    
    let input: Vec<&str> = include_str!("../data/day_10.txt").trim().lines().collect();
    
    let a = part_a(&input);
    println!("Part A result: {}", a);
    let b = part_b(&input);
    println!("Part B result: {}", b);
}

fn part_a(input: &Vec<&str>) -> i32 {
    let mut score = 0;

    for i in input {
        let mut stack = Vec::new();

        for c in i.chars() {
            if ['(', '[', '{', '<'].contains(&c) {
                stack.push(c);
            } else {
                let l = *stack.last().unwrap();
                if (c == ')' && l == '(') || 
                   (c == ']' && l == '[') || 
                   (c == '}' && l == '{') || 
                   (c == '>' && l == '<') { stack.pop(); }
                else {
                    score += match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0
                    };
                    break;
                }
            }
        }
    }

    score
}

fn part_b(input: &Vec<&str>) -> i64 {
    let mut scores: Vec<i64> = Vec::new();
    let input = input.clone();

    let input = input.iter().filter(|i| {
        let mut stack = Vec::new();

        for c in i.chars() {
            if ['(', '[', '{', '<'].contains(&c) {
                stack.push(c);
            } else {
                let l = *stack.last().unwrap();
                if (c == ')' && l == '(') || 
                   (c == ']' && l == '[') || 
                   (c == '}' && l == '{') || 
                   (c == '>' && l == '<') { stack.pop(); }
                else {
                    return false;
                }
            }
        }

        true
    });

    for i in input {
        let mut stack = Vec::new();

        for c in i.chars() {
            if ['(', '[', '{', '<'].contains(&c) {
                stack.push(c);
            } else {
                let l = *stack.last().unwrap();
                if (c == ')' && l == '(') || 
                   (c == ']' && l == '[') || 
                   (c == '}' && l == '{') || 
                   (c == '>' && l == '<') { stack.pop(); }
            }
        }

        stack.reverse();

        let mut score = 0;
        for i in stack {
            score *= 5;
            score += match i {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0
            };
        }
        scores.push(score);
    }

    scores.sort();
    scores[scores.len()/2]
}