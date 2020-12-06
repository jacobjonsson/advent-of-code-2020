extern crate time;
use time::Instant;

#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day5() {
    let start = Instant::now();
    let input = input_loader::read_input("src/day_5/input.txt");

    let mut max_seat_id = 0;
    let mut seat_ids: Vec<i32> = Vec::new();

    for row in input.trim_end().split("\n").collect::<Vec<&str>>().iter() {
        let row_number = get_row(&row[0..7]);
        let column_number = get_column(&row[7..10]);

        let seat_id = get_seat_id(row_number, column_number);

        seat_ids.push(seat_id);

        if seat_id >= max_seat_id {
            max_seat_id = seat_id;
        }
    }

    seat_ids.sort();

    let mut my_seat_id = 0;
    for (i, s) in seat_ids.iter().enumerate() {
        if i > 1 {
            if seat_ids[i - 1] != s - 1 {
                my_seat_id = s - 1;
            }
        }
    }

    println!("Max seat id: {}", max_seat_id);
    println!("My seat id: {}", my_seat_id);

    println!(
        "Took {} seconds to complete",
        start.elapsed().as_seconds_f32()
    );
}

fn get_row(rows_indicators: &str) -> i32 {
    let mut lower_bound = 0;
    let mut upper_bound = 127;

    for row_indicator in rows_indicators.chars() {
        let diff: f64 = (upper_bound as f64 - lower_bound as f64) / 2 as f64;

        if row_indicator == 'F' {
            upper_bound = (upper_bound as f64 - diff).floor() as i32;
        } else if row_indicator == 'B' {
            lower_bound = (lower_bound as f64 + diff).ceil() as i32;
        } else {
            panic!("{} is not a row indicator", row_indicator);
        }
    }

    if lower_bound != upper_bound {
        panic!("{} and {} does not match", lower_bound, upper_bound);
    }

    return lower_bound;
}

fn get_column(column_indicators: &str) -> i32 {
    let mut lower_bound = 0;
    let mut upper_bound = 7;

    for column_indicator in column_indicators.chars() {
        let diff: f64 = (upper_bound as f64 - lower_bound as f64) / 2 as f64;

        if column_indicator == 'L' {
            upper_bound = (upper_bound as f64 - diff).floor() as i32;
        } else if column_indicator == 'R' {
            lower_bound = (lower_bound as f64 + diff).ceil() as i32;
        } else {
            panic!("{} is not a column indicator", column_indicator);
        }
    }

    if lower_bound != upper_bound {
        panic!("{} and {} does not match", lower_bound, upper_bound);
    }

    return lower_bound;
}

fn get_seat_id(row: i32, column: i32) -> i32 {
    return row * 8 + column;
}
