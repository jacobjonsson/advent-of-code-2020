#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day5() {
    let input = input_loader::read_input("src/day_5/input.txt");
    let groups = input.trim_end().split("\n").collect::<Vec<&str>>();

    let seat_ids = groups
        .iter()
        .map(|&x| get_seat_id(get_row(&x[0..7]), get_column(&x[7..10])))
        .collect::<Vec<i32>>();

    let mut sorted = seat_ids.clone();
    sorted.sort();

    let mut my_seat_id = 0;
    for (i, s) in sorted.iter().enumerate() {
        if i > 1 {
            if sorted[i - 1] != s - 1 {
                my_seat_id = s - 1;
            }
        }
    }

    match seat_ids.iter().max() {
        Some(v) => println!("[DAY 5] Result for part 1: {}", v),
        None => panic!("Failed to get max of seat_ids"),
    };
    println!("[DAY 5] Result for part 2: {}", my_seat_id);
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
