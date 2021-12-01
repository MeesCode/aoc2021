
use std::env;

mod day_01;

fn main() {
    let arg: Option<String> = env::args().nth(1);

    if let Some(day) = arg {

        let tasks = [
            day_01::run,
        ];

        if day == "all" {
            for (index, task) in tasks.iter().enumerate() {
                println!("Day {}", index+1);
                task();
                if index < tasks.len()-1{ println!(); }
            }
        } else {
            if let Ok(index) = day.parse::<usize>() {
                if index <= tasks.len() {
                    println!("Day {}", index);
                    tasks[index-1]();
                } else if index > 25 {
                    println!("christmas is over already");
                } else {
                    println!("day not implemented");
                }
            } else {
                println!("not a valid argument");
            }
        }

    } else {
        println!("not enough arguments");
    }

}
