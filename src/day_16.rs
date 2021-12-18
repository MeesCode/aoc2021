
pub fn main(){    
    let input = include_str!("../data/day_16.txt").trim();    
    let (_, a, b) = part_ab(&hex2bin(input), 0);
    println!("Part 1: {}", a);
    println!("Part 2: {}", b);
}

fn part_ab(bin: &str, index: usize) -> (usize, i64, i64) {
    if bin.len() < index + 11 { return (index, 0, 0); }

    let value;
    let mut index = index;
    let mut version = bin2int(&bin[index+0..index+3]);
    let type_id = bin2int(&bin[index+3..index+6]);
    index += 6;

    if type_id == 4 {
        let mut value_string = String::new();
        loop {
            value_string.push_str(&bin[index+1..index+5]);
            if bin.chars().nth(index).unwrap() == '0' { index += 5; break; }
            index += 5;
        }
        value = bin2int(&value_string);
    } else {
        let mut sub_values = Vec::new();

        if bin.chars().nth(index).unwrap() == '0' { 
            index += 1;
            let max_index = index + bin2int(&bin[index..index+15]) as usize + 15;
            index += 15;
            while index < max_index {
                let (new_index, sub_version, sub_value) = part_ab(bin, index);
                index = new_index;
                version += sub_version;
                sub_values.push(sub_value);
            }
        } else {
            index += 1;
            let subs = bin2int(&bin[index..index+11]);
            index += 11;
            for _ in 0..subs {
                let (new_index, sub_version, sub_value) = part_ab(bin, index);
                index = new_index;
                version += sub_version;
                sub_values.push(sub_value);
            }
        }

        value = match type_id {
            0 => sub_values.iter().fold(0, |a,c| a + c),
            1 => sub_values.iter().fold(1, |a,c| a * c),
            2 => sub_values.iter().fold(i64::MAX, |a,&c| if c < a { c } else { a }),
            3 => sub_values.iter().fold(0, |a,&c| if c > a { c } else { a }),
            5 => if sub_values[0] > sub_values[1] { 1 } else { 0 },
            6 => if sub_values[0] < sub_values[1] { 1 } else { 0 },
            7 => if sub_values[0] == sub_values[1] { 1 } else { 0 },
            _ => 0
        };

    }

    (index, version, value)
}

fn bin2int(bin: &str) -> i64 {
    bin.chars().rev().fold((0, 0), |acc, x| (acc.0 + if x == '1' { 1 << acc.1 } else { 0 }, acc.1+1)).0
}

fn hex2bin(string: &str) -> String {
    let mut res: String = String::new();
    for i in string.chars() {
        let s = match i {
            '0' => "0000", '1' => "0001", '2' => "0010", '3' => "0011", '4' => "0100",
            '5' => "0101", '6' => "0110", '7' => "0111", '8' => "1000", '9' => "1001",
            'A' => "1010", 'B' => "1011", 'C' => "1100", 'D' => "1101", 'E' => "1110",
            'F' => "1111",  _  => "error"
        };
        res.push_str(s);
    }
    res
}