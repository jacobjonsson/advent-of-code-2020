use std::collections::HashMap;

#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day15() {
    let input = input_loader::read_input("src/day_15/input.txt");

    let part_1_result = part_1(&input);
    println!("[DAY 15] Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("[DAY 15] Result for part 2: {}", part_2_result);
}

fn part_1(input: &str) -> i32 {
    let mut visited: HashMap<i32, Vec<usize>> = HashMap::new();

    let mut last_spoken: i32 = 0;
    for (idx, num) in input.split(",").enumerate() {
        let parsed = num.parse::<i32>().unwrap();
        visited.insert(parsed, vec![idx + 1]);
        last_spoken = parsed;
    }

    for idx in (input.split(",").collect::<Vec<&str>>().len() + 1)..2021 {
        if !visited.contains_key(&last_spoken) {
            last_spoken = 0;
            if visited.contains_key(&0) {
                visited.get_mut(&0).unwrap().push(idx);
            } else {
                visited.insert(0, vec![idx]);
            }
            continue;
        }

        let value = match visited.get(&last_spoken) {
            Some(v) => v,
            None => panic!("Could not get value for {}", last_spoken),
        };

        if value.len() == 1 {
            last_spoken = 0;
            if visited.contains_key(&0) {
                visited.get_mut(&0).unwrap().push(idx);
            } else {
                visited.insert(0, vec![idx]);
            }
            continue;
        }

        let diff = (value[value.len() - 1] - value[value.len() - 2]) as i32;
        last_spoken = diff;
        if visited.contains_key(&diff) {
            visited.get_mut(&diff).unwrap().push(idx);
        } else {
            visited.insert(diff, vec![idx]);
        }
    }

    return last_spoken;
}

fn part_2(input: &str) -> i32 {
    let mut visited: HashMap<i32, Vec<usize>> = HashMap::new();

    let mut last_spoken: i32 = 0;
    for (idx, num) in input.split(",").enumerate() {
        let parsed = num.parse::<i32>().unwrap();
        visited.insert(parsed, vec![idx + 1]);
        last_spoken = parsed;
    }

    for idx in (input.split(",").collect::<Vec<&str>>().len() + 1)..30000001 {
        if !visited.contains_key(&last_spoken) {
            last_spoken = 0;
            if visited.contains_key(&0) {
                visited.get_mut(&0).unwrap().push(idx);
            } else {
                visited.insert(0, vec![idx]);
            }
            continue;
        }

        let value = match visited.get(&last_spoken) {
            Some(v) => v,
            None => panic!("Could not get value for {}", last_spoken),
        };

        if value.len() == 1 {
            last_spoken = 0;
            if visited.contains_key(&0) {
                visited.get_mut(&0).unwrap().push(idx);
            } else {
                visited.insert(0, vec![idx]);
            }
            continue;
        }

        let diff = (value[value.len() - 1] - value[value.len() - 2]) as i32;
        last_spoken = diff;
        if visited.contains_key(&diff) {
            visited.get_mut(&diff).unwrap().push(idx);
        } else {
            visited.insert(diff, vec![idx]);
        }
    }

    return last_spoken;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(436, part_1(&String::from("0,3,6")));
        assert_eq!(1, part_1(&String::from("1,3,2")));
        assert_eq!(10, part_1(&String::from("2,1,3")));
        assert_eq!(27, part_1(&String::from("1,2,3")));
        assert_eq!(78, part_1(&String::from("2,3,1")));
        assert_eq!(438, part_1(&String::from("3,2,1")));
        assert_eq!(1836, part_1(&String::from("3,1,2")));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(175594, part_2(&String::from("0,3,6")));
    }
}
