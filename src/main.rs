extern crate time;
use std::env;
use time::Instant;

#[path = "./day_1/day_1.rs"]
mod day_1;
#[path = "./day_10/day_10.rs"]
mod day_10;
#[path = "./day_11/day_11.rs"]
mod day_11;
#[path = "./day_12/day_12.rs"]
mod day_12;
#[path = "./day_13/day_13.rs"]
mod day_13;
#[path = "./day_14/day_14.rs"]
mod day_14;
#[path = "./day_15/day_15.rs"]
mod day_15;
#[path = "./day_16/day_16.rs"]
mod day_16;
#[path = "./day_17/day_17.rs"]
mod day_17;
#[path = "./day_18/day_18.rs"]
mod day_18;
#[path = "./day_19/day_19.rs"]
mod day_19;
#[path = "./day_2/day_2.rs"]
mod day_2;
#[path = "./day_20/day_20.rs"]
mod day_20;
#[path = "./day_21/day_21.rs"]
mod day_21;
#[path = "./day_22/day_22.rs"]
mod day_22;
#[path = "./day_23/day_23.rs"]
mod day_23;
#[path = "./day_24/day_24.rs"]
mod day_24;
#[path = "./day_25/day_25.rs"]
mod day_25;
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
        "11" => day_11::day11(),
        "12" => day_12::day12(),
        "13" => day_13::day13(),
        "14" => day_14::day14(),
        "15" => day_15::day15(),
        "16" => day_16::day16(),
        "17" => day_17::day17(),
        "18" => day_18::day18(),
        "19" => day_19::day19(),
        "20" => day_20::day20(),
        "21" => day_21::day21(),
        "22" => day_22::day22(),
        "23" => day_23::day23(),
        "24" => day_24::day24(),
        "25" => day_25::day25(),
        "ALL" => {
            day_1::day1();
            println!("------------------------------------");
            day_2::day2();
            println!("------------------------------------");
            day_3::day3();
            println!("------------------------------------");
            day_4::day4();
            println!("------------------------------------");
            day_5::day5();
            println!("------------------------------------");
            day_6::day6();
            println!("------------------------------------");
            day_7::day7();
            println!("------------------------------------");
            day_8::day8();
            println!("------------------------------------");
            day_9::day9();
            println!("------------------------------------");
            day_10::day10();
            println!("------------------------------------");
            day_11::day11();
            println!("------------------------------------");
            day_12::day12();
            println!("------------------------------------");
            day_13::day13();
            println!("------------------------------------");
            day_14::day14();
            println!("------------------------------------");
            day_15::day15();
            println!("------------------------------------");
            day_16::day16();
            println!("------------------------------------");
            day_17::day17();
            println!("------------------------------------");
            day_18::day18();
            println!("------------------------------------");
            day_19::day19();
            println!("------------------------------------");
            day_20::day20();
            println!("------------------------------------");
            day_21::day21();
            println!("------------------------------------");
            day_22::day22();
            println!("------------------------------------");
            day_23::day23();
            println!("------------------------------------");
            day_24::day24();
            println!("------------------------------------");
            day_25::day25();
            println!("------------------------------------");
        }
        _ => panic!("Provided day does not exist"),
    };

    println!(
        "Program finished in: {} seconds",
        start.elapsed().as_seconds_f64()
    );
}
