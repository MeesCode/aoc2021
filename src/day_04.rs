use regex::Regex;

#[derive(Clone, Copy)]
struct Card {
    numbers: [[i32; 5]; 5],
}

impl Card {
    fn remove(&mut self, num: i32) {
        for y in 0..5 {
            for x in 0..5 {
                if self.numbers[y][x] == num {
                    self.numbers[y][x] = -1;
                }
            }
        }
    }

    fn won(&self) -> bool {
        for y in self.numbers {
            if y.eq(&[-1; 5]) { return true }
            'outer: for x in 0..5 {
                for y in 0..5 {
                    if self.numbers[y][x] != -1 { continue 'outer; };
                }
                return true;
            }
        }
        false
    }

    fn score(&self) -> i32 {
        self.numbers.iter().fold(0, |acc, i| acc + i.iter().fold(0, |acc1, &j| acc1 + if j != -1 { j } else { 0 }))
    }
}

pub fn run(){    
    let input = include_str!("../data/day_04.txt").trim();
    let numbers: Vec<i32> = input.lines().next().unwrap().split(",").map(|x| x.parse().unwrap()).collect();
    let mut cards: Vec<Card> = Vec::new();

    let re = Regex::new(r"(\d+)\s*(\d+)\s*(\d+)\s*(\d+)\s*(\d+)\n").unwrap();
    let mut row = 0;
    let mut cur_card = Card{ numbers: [[0; 5]; 5] };
    for cap in re.captures_iter(input) {
        cur_card.numbers[row] = [
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
            cap[4].parse().unwrap(),
            cap[5].parse().unwrap(),
        ];
        row += 1;
        if row == 5 {
            cards.push(cur_card.clone());
            row = 0;
        }
    }

    let a = part_a(&numbers, &mut cards);
    println!("Part A result: {}", a);
    let b = part_b(&numbers, &mut cards);
    println!("Part B result: {}", b);
}

fn part_a(numbers: &Vec<i32>, cards: &Vec<Card>) -> i32 {
    let mut cards = cards.clone();
    for n in numbers {
        for c in &mut cards { c.remove(*n);}
        if let Some(found) = cards.iter().find(|x| x.won()) {
            return n * found.score()
        }
    }
    0
}

fn part_b(numbers: &Vec<i32>, cards: &Vec<Card>) -> i32 {
    let mut cards = cards.clone();
    for n in numbers {
        for c in &mut cards { c.remove(*n); }
        if cards.len() == 1 && cards[0].won() { 
            return n * cards[0].score()
        }
        cards = cards.iter().filter(|c| !c.won()).map(|x| *x).collect();
    }
    0
}
