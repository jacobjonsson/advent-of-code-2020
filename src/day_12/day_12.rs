#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day12() {
    let input = input_loader::read_input("src/day_12/input.txt");

    let part_1_result = part_1(&input);
    println!("[DAY 12] Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("[DAY 12] Result for part 2: {}", part_2_result);
}

#[derive(Debug, PartialEq)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    value: i32,
}

fn parse_action(input: &str) -> Action {
    let action = match input.chars().nth(0) {
        Some(t) => t,
        None => panic!("Unable to get the action from {}", input),
    };

    return match action {
        'N' => Action::North,
        'S' => Action::South,
        'E' => Action::East,
        'W' => Action::West,
        'L' => Action::Left,
        'R' => Action::Right,
        'F' => Action::Forward,
        _ => panic!("{} is not a valid action", action),
    };
}

fn parse_value(input: &str) -> i32 {
    return match &input[1..].parse() {
        Ok(t) => *t,
        Err(_err) => panic!("Unable to parse {} to an int", &input[1..]),
    };
}

fn get_next_right_direction(current_direction: Direction, times: i32) -> Direction {
    match current_direction {
        Direction::East => {
            return match times {
                1 => Direction::South,
                2 => Direction::West,
                3 => Direction::North,
                4 => Direction::East,
                _ => panic!("Times is not valid {}", times),
            }
        }
        Direction::West => {
            return match times {
                1 => Direction::North,
                2 => Direction::East,
                3 => Direction::South,
                4 => Direction::West,
                _ => panic!("Times is not valid {}", times),
            }
        }
        Direction::North => {
            return match times {
                1 => Direction::East,
                2 => Direction::South,
                3 => Direction::West,
                4 => Direction::North,
                _ => panic!("Times is not valid {}", times),
            }
        }
        Direction::South => {
            return match times {
                1 => Direction::West,
                2 => Direction::North,
                3 => Direction::East,
                4 => Direction::South,
                _ => panic!("Times is not valid {}", times),
            }
        }
    };
}

fn get_next_left_direction(current_direction: Direction, times: i32) -> Direction {
    match current_direction {
        Direction::East => {
            return match times {
                1 => Direction::North,
                2 => Direction::West,
                3 => Direction::South,
                4 => Direction::East,
                _ => panic!("Times is not valid {}", times),
            }
        }
        Direction::West => {
            return match times {
                1 => Direction::South,
                2 => Direction::East,
                3 => Direction::North,
                4 => Direction::West,
                _ => panic!("Times is not valid {}", times),
            }
        }
        Direction::North => {
            return match times {
                1 => Direction::West,
                2 => Direction::South,
                3 => Direction::East,
                4 => Direction::North,
                _ => panic!("Times is not valid {}", times),
            }
        }
        Direction::South => {
            return match times {
                1 => Direction::East,
                2 => Direction::North,
                3 => Direction::West,
                4 => Direction::South,
                _ => panic!("Times is not valid {}", times),
            }
        }
    };
}

fn part_1(input: &str) -> i32 {
    let instructions = input
        .trim_end()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| Instruction {
            action: parse_action(x),
            value: parse_value(x),
        })
        .collect::<Vec<Instruction>>();

    let mut current_direction = Direction::East;
    let mut x = 0; // West + East
    let mut y = 0; // North + South
    for instruction in instructions {
        match instruction.action {
            Action::Forward => match current_direction {
                Direction::North => y += instruction.value,
                Direction::East => x += instruction.value,
                Direction::South => y -= instruction.value,
                Direction::West => x -= instruction.value,
            },
            Action::North => y += instruction.value,
            Action::East => x += instruction.value,
            Action::South => y -= instruction.value,
            Action::West => x -= instruction.value,
            Action::Left => {
                current_direction =
                    get_next_left_direction(current_direction, instruction.value / 90);
            }
            Action::Right => {
                current_direction =
                    get_next_right_direction(current_direction, instruction.value / 90);
            }
        }
    }
    return x.abs() + y.abs();
}

struct Waypoint {
    x: i32,
    y: i32,
}

fn part_2(input: &str) -> i32 {
    let instructions = input
        .trim_end()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| Instruction {
            action: parse_action(x),
            value: parse_value(x),
        })
        .collect::<Vec<Instruction>>();

    let mut waypoint = Waypoint { x: 10, y: 1 };

    let mut x = 0;
    let mut y = 0;

    for instruction in instructions {
        match instruction.action {
            Action::Forward => {
                x += waypoint.x * instruction.value;
                y += waypoint.y * instruction.value;
            }
            Action::North => waypoint.y += instruction.value,
            Action::East => waypoint.x += instruction.value,
            Action::South => waypoint.y -= instruction.value,
            Action::West => waypoint.x -= instruction.value,
            Action::Left => {
                let mut degrees = instruction.value.clone();
                while degrees > 0 {
                    let tmp_x = waypoint.x.clone();
                    let tmp_y = waypoint.y.clone();
                    waypoint.x = -tmp_y;
                    waypoint.y = tmp_x;
                    degrees -= 90;
                }
            }
            Action::Right => {
                let mut degrees = instruction.value.clone();
                while degrees > 0 {
                    let tmp_x = waypoint.x.clone();
                    let tmp_y = waypoint.y.clone();
                    waypoint.x = tmp_y;
                    waypoint.y = -tmp_x;
                    degrees -= 90;
                }
            }
        }
    }
    return x.abs() + y.abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "F10
N3
F7
R90
F11"
        .to_string();
        assert_eq!(25, part_1(&input));
    }

    #[test]
    fn test_part_2() {
        let input = "F10
N3
F7
R90
F11"
        .to_string();
        assert_eq!(286, part_2(&input));
    }
}
