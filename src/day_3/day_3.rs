#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day3() {
    let input = input_loader::read_input("src/day_3/input.txt");
    let lines = input.trim_end().split("\n").collect::<Vec<&str>>();

    let slope_1 = traverse_slope(1, 1, &lines);
    let slope_2 = traverse_slope(3, 1, &lines);
    let slope_3 = traverse_slope(5, 1, &lines);
    let slope_4 = traverse_slope(7, 1, &lines);
    let slope_5 = traverse_slope(1, 2, &lines);

    println!("[DAY 3] Result for part 1: {}", slope_2);

    println!(
        "[DAY 3] Result for part 2: {}",
        slope_1 * slope_2 * slope_3 * slope_4 * slope_5
    );
}

fn traverse_slope(x_incrementor: i32, y_incrementor: i32, lines: &Vec<&str>) -> i64 {
    let mut current_x: i32 = 0;
    let mut found_trees: i64 = 0;

    let max_width = get_potential_max_width(&(lines.len() as i32), &x_incrementor);

    for x in 0..lines.len() {
        if x as i32 % y_incrementor != 0 {
            continue;
        }

        let max_width_multiplier =
            get_potential_max_width_multiplier(&(lines[x].len() as i32), &max_width);
        let new_content = increment_line_by_x(&lines[x], max_width_multiplier);
        let c = find_char_at_index(&new_content, current_x);
        if is_tree(c) {
            found_trees += 1;
        }
        current_x = increment_x(&current_x, &x_incrementor);
    }

    return found_trees;
}

fn increment_x(x: &i32, x_incrementor: &i32) -> i32 {
    return x + x_incrementor;
}

fn is_tree(c: char) -> bool {
    return c == '#';
}

fn find_char_at_index(line: &str, index: i32) -> char {
    return line.chars().nth(index as usize).unwrap();
}

fn get_potential_max_width(length: &i32, incrementor: &i32) -> i32 {
    return length * (incrementor + incrementor);
}

fn get_potential_max_width_multiplier(line_width: &i32, max_width: &i32) -> i32 {
    return (max_width + line_width) / line_width;
}

fn increment_line_by_x(line: &str, x: i32) -> String {
    let mut new_line = "".to_string();
    for _ in 0..x {
        new_line.push_str(line);
    }

    return new_line;
}
