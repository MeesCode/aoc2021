use regex::Regex;

#[derive(Clone, Copy)]
struct Card {
    numbers: [[i32; 5]; 5],
    won: bool
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
        if self.won { return true }
        for y in self.numbers {
            if y.eq(&[-1; 5]) { /* self.won = true; */ return true }
            
            'outer: for x in 0..5 {
                for y in 0..5 {
                    if self.numbers[y][x] != -1 { continue 'outer; };
                }
                return true;
            }
            
        }
        false
    }

    fn new() -> Card{
        Card {
            numbers: [[0; 5]; 5],
            won: false
        }
    }
}

pub fn run(){    
    let mut input = include_str!("../data/day_04.txt").trim().lines();

    let numbers: Vec<i32> = input.next().unwrap().split(",").map(|x| x.parse().unwrap()).collect();
    let mut cards: Vec<Card> = Vec::new();

    let re = Regex::new(r"(\d+)\s*(\d+)\s*(\d+)\s*(\d+)\s*(\d+)\n").unwrap();

    input.next();

    let mut row = 0;
    let mut cur_card = Card::new();
    for cap in re.captures_iter(include_str!("../data/day_04.txt")) {
        cur_card.numbers[row] = [
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
            cap[4].parse().unwrap(),
            cap[5].parse().unwrap(),
        ];

        if row == 4 {
            cards.push(cur_card.clone());
            row = 0;
            continue;
        }

        row += 1;
    }

    let a = part_a(&numbers, &mut cards);
    println!("Part A result: {}", a);
    let b = part_b(&numbers, &mut cards);
    println!("Part B result: {}", b);
}

fn part_a(numbers: &Vec<i32>, cards: &Vec<Card>) -> i32 {

    let mut cards = cards.clone();
    let mut round = 0;
    let mut card = 0;

    'outer: for n in numbers {
        for c in &mut cards {
            c.remove(*n);
        }

        for (index, i) in cards.iter().enumerate() {
            if i.won() {
                card = index;
                round = *n;
                break 'outer;
            }
        }
    }

    let mut count = 0;
    for y in cards[card].numbers {
        for x in y {
            count += if x != -1 { x } else { 0 }
        }
    }

    count * round
}

fn part_b(numbers: &Vec<i32>, cards: &Vec<Card>) -> i32 {

    let mut cards = cards.clone();
    let mut round = 0;

    for n in numbers {

        for c in &mut cards {
            c.remove(*n);
        }

        if cards.len() == 1 && cards[0].won() { round = *n; break; }

        cards = cards.iter().filter(|c| !c.won()).map(|x| *x).collect();
    }

    // println!("round {}", round);

    let mut count = 0;
    for y in cards[0].numbers {
        // println!("{:?}", y);
        for x in y {
            count += if x != -1 { x } else { 0 }
        }
    }

    // println!("count {}", count);

    count * round
}
