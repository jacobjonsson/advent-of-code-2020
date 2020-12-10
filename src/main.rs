extern crate time;
use std::env;
use time::Instant;
#[path = "./day_1/day_1.rs"]
mod day_1;
#[path = "./day_10/day_10.rs"]
mod day_10;
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
#[path = "./day_7/day_7.rs"]
mod day_7;
#[path = "./day_8/day_8.rs"]
mod day_8;
#[path = "./day_9/day_9.rs"]
mod day_9;

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
        "7" => day_7::day7(),
        "8" => day_8::day8(),
        "9" => day_9::day9(),
        "10" => day_10::day10(),
        "ALL" => {
            day_1::day1();
            day_2::day2();
            day_3::day3();
            day_4::day4();
            day_5::day5();
            day_6::day6();
            day_7::day7();
            day_8::day8();
            day_9::day9();
            day_10::day10();
        }
        _ => panic!("Provided day does not exist"),
    };

    println!(
        "Program finished in: {} seconds",
        start.elapsed().as_seconds_f64()
    );
}
