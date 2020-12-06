#[path = "../input_loader/input_loader.rs"]
mod input_loader;

#[derive(Debug)]
struct Policy {
    lower_bound: i32,
    upper_bound: i32,
    character: char,
    password: String,
}

fn extract_policy(value: &str) -> Policy {
    let splitted = value.split(":").collect::<Vec<&str>>();
    let policy = splitted[0].trim();
    let password = splitted[1].trim();
    let policy_tokens: Vec<&str> = policy.split(" ").collect();
    let boundaries = policy_tokens[0].split("-").collect::<Vec<&str>>();
    let lower_bound = boundaries[0];
    let upper_bound = boundaries[1];
    let character = policy_tokens[1];

    return Policy {
        lower_bound: String::from(lower_bound).parse::<i32>().unwrap(),
        upper_bound: String::from(upper_bound).parse::<i32>().unwrap(),
        character: character.chars().collect::<Vec<char>>()[0],
        password: password.to_string(),
    };
}

fn follows_policy_part_1(policy: &Policy) -> bool {
    let count = policy
        .password
        .chars()
        .filter(|&x| x == policy.character)
        .count() as i32;

    return count >= policy.lower_bound && count <= policy.upper_bound;
}

fn follows_policy_part_2(policy: &Policy) -> bool {
    let lower_bound_char = policy
        .password
        .chars()
        .nth(policy.lower_bound as usize - 1)
        .unwrap();
    let upper_bound_char = policy
        .password
        .chars()
        .nth(policy.upper_bound as usize - 1)
        .unwrap();

    if lower_bound_char == upper_bound_char {
        return false;
    }

    if lower_bound_char == policy.character {
        return true;
    }

    if upper_bound_char == policy.character {
        return true;
    }

    return false;
}

fn part_1(lines: &[&str]) -> i32 {
    return lines
        .iter()
        .map(|x| extract_policy(x))
        .filter(|x| follows_policy_part_1(x))
        .count() as i32;
}

fn part_2(lines: &[&str]) -> i32 {
    return lines
        .iter()
        .map(|x| extract_policy(x))
        .filter(|x| follows_policy_part_2(x))
        .count() as i32;
}

pub fn day2() {
    let input = input_loader::read_input("src/day_2/input.txt");
    let lines = input.trim_end().split("\n").collect::<Vec<&str>>();

    let part_1_result = part_1(&lines);
    let part_2_result = part_2(&lines);

    println!("[DAY 2] Result for part 1: {}", part_1_result);
    println!("[DAY 2] Result for part 2: {}", part_2_result);
}
