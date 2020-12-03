use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Line {
    original_length: i32,
    content: String,
}

pub fn day3() {
    let lines = read_input();

    let slope_1 = traverse_map(1, 1, &lines);
    let slope_2 = traverse_map(3, 1, &lines);
    let slope_3 = traverse_map(5, 1, &lines);
    let slope_4 = traverse_map(7, 1, &lines);
    let slope_5 = traverse_map(1, 2, &lines);

    println!("Found trees for slope Right 1, down 1: {}", slope_1);
    println!("Found trees for slope Right 3, down 1: {}", slope_2);
    println!("Found trees for slope Right 5, down 1: {}", slope_3);
    println!("Found trees for slope Right 7, down 1: {}", slope_5);
    println!("Found trees for slope Right 1, down 2: {}", slope_5);

    println!(
        "Sum of these are: {}",
        slope_1 * slope_2 * slope_3 * slope_4 * slope_5
    );
}

fn traverse_map(x_incrementer: i32, y_incrementer: i32, lines: &Vec<Line>) -> i64 {
    let mut current_x: i32 = 0;
    let mut found_trees: i64 = 0;

    let max_width = get_potential_max_width(&(lines.len() as i32), &x_incrementer);

    for x in 0..lines.len() {
        if x as i32 % y_incrementer != 0 {
            continue;
        }

        let max_width_multiplier =
            get_potential_max_width_multiplier(&lines[x].original_length, &max_width);
        let new_content = increment_line_by_x(&lines[x].content, max_width_multiplier);
        let c = find_char_at_index(&new_content, current_x);
        if is_tree(c) {
            found_trees += 1;
        }
        current_x = increment_x(&current_x, &x_incrementer);
    }

    return found_trees;
}

fn increment_x(x: &i32, x_incrementer: &i32) -> i32 {
    return x + x_incrementer;
}

fn is_tree(c: char) -> bool {
    return c == '#';
}

fn find_char_at_index(line: &str, index: i32) -> char {
    return line.chars().nth(index as usize).unwrap();
}

fn get_potential_max_width(length: &i32, incrementer: &i32) -> i32 {
    return length * (incrementer + incrementer);
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

fn read_input() -> Vec<Line> {
    let f = File::open("src/day_3/input.txt").unwrap();
    let file = BufReader::new(&f);
    let mut vec = Vec::new();
    for line in file.lines() {
        let l = line.unwrap();
        vec.push(Line {
            original_length: l.len() as i32,
            content: l,
        });
    }
    return vec;
}
