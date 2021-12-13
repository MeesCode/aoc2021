
use std::env;
use std::time::Instant;

mod day_01; mod day_02; mod day_03; mod day_04; mod day_05; 
mod day_06; mod day_07; mod day_08; mod day_09; mod day_10;
mod day_11; mod day_12; mod day_13;

fn do_task(day: usize, task: &fn()) {
    println!("== Day {} ==", day);
    let task_time = Instant::now();
    task();
    println!("Time: {:?}", task_time.elapsed());
}

fn main() {
    let arg: Option<String> = env::args().nth(1);

    if let Some(day) = arg {

        let tasks = [
            day_01::main, day_02::main, day_03::main, day_04::main, day_05::main, 
            day_06::main, day_07::main, day_08::main, day_09::main, day_10::main,
            day_11::main, day_12::main, day_13::main,
        ];

        if day == "all" {
            let total_time = Instant::now();
            for (day, task) in tasks.iter().enumerate() {
                do_task(day+1, task);
                println!();
            }
            println!("Total time: {:?}", total_time.elapsed());
        } else {
            if let Ok(index) = day.parse::<usize>() {
                if index <= tasks.len() {
                    do_task(index, &tasks[index-1]);
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
