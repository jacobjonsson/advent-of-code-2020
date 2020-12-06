extern crate time;
use std::env;
use time::Instant;

#[path = "./day_1/day_1.rs"]
mod day_1;

#[path = "./day_2/day_2.rs"]
mod day_2;

#[path = "./day_3/day_3.rs"]
mod day_3;

#[path = "./day_4/day_4.rs"]
mod day_4;

#[path = "./day_5/day_5.rs"]
mod day_5;

#[path = "./day_6/day_6.rs"]
mod day_6;

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();

    let day = &args[1];

    match day.as_str() {
        "1" => day_1::day1(),
        "2" => day_2::day2(),
        "3" => day_3::day3(),
        "4" => day_4::day4(),
        "5" => day_5::day5(),
        "6" => day_6::day6(),
        "ALL" => {
            day_1::day1();
            day_2::day2();
            day_3::day3();
            day_4::day4();
            day_5::day5();
            day_6::day6();
        }
        _ => panic!("Provided day does not exist"),
    };

    println!(
        "Program finished in: {} seconds",
        start.elapsed().as_seconds_f64()
    );
}
