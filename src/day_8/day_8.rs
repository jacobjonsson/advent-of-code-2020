#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day8() {
    let input = input_loader::read_input("src/day_8/input.txt");

    let part_1_result = part_1(&input);
    println!("[DAY 8] Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("[DAY 8] Result for part 2: {}", part_2_result);
}

fn parse_value(str: &str) -> i32 {
    return str.split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
}

fn part_1(input: &str) -> i32 {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut current_index = 0;
    let mut visited: Vec<usize> = Vec::new();
    let mut acc = 0;

    loop {
        if visited.contains(&current_index) {
            break;
        }

        visited.push(current_index);
        if lines[current_index].contains("nop") {
            current_index += 1;
            continue;
        }

        if lines[current_index].contains("acc") {
            let value: i32 = parse_value(lines[current_index]);
            acc += value;
            current_index += 1;
            continue;
        }

        if lines[current_index].contains("jmp") {
            let value: i32 = parse_value(lines[current_index]);
            current_index = (current_index as i32 + value) as usize;
            continue;
        }
    }

    return acc;
}

fn try_program(lines: &Vec<&str>) -> i32 {
    let mut current_index = 0;
    let mut visited: Vec<usize> = Vec::new();
    let mut acc = 0;

    loop {
        if visited.contains(&current_index) {
            return -1;
        }

        if current_index == lines.len() {
            return acc;
        }

        visited.push(current_index);
        if lines[current_index].contains("nop") {
            current_index += 1;
            continue;
        }

        if lines[current_index].contains("acc") {
            let value: i32 = parse_value(lines[current_index]);
            acc += value;
            current_index += 1;
            continue;
        }

        if lines[current_index].contains("jmp") {
            let value: i32 = parse_value(lines[current_index]);

            current_index = (current_index as i32 + value) as usize;
            continue;
        }
    }
}

fn part_2(input: &str) -> i32 {
    let lines: Vec<&str> = input.split("\n").collect();

    for (idx, line) in lines.iter().enumerate() {
        let mut current_lines = lines.to_vec();
        let mut new_line = String::new();

        if line.contains("acc") {
            continue;
        }

        if line.contains("jmp") {
            new_line = line.replace("jmp", "nop");
        } else if line.contains("nop") {
            new_line = line.replace("nop", "jmp");
        }

        current_lines[idx] = new_line.as_str();

        let x = try_program(&current_lines);
        if x != -1 {
            return x;
        }
    }

    panic!("Failed...")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            .to_string();
        assert_eq!(5, part_1(&input));
    }

    #[test]
    fn test_part_2() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            .to_string();
        assert_eq!(8, part_2(&input));
    }
}
