extern crate time;
use std::collections::HashMap;
use time::Instant;

#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day6() {
    let start = Instant::now();

    let input = input_loader::read_input("src/day_6/input.txt");

    let part_1_result = part_1(&input);
    println!("Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("Result for part 2: {}", part_2_result);

    println!(
        "Took {} seconds to complete",
        start.elapsed().as_seconds_f32()
    );
}

fn part_1(input: &str) -> i32 {
    let groups: Vec<&str> = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.trim())
        .collect();
    let mut count: i32 = 0;
    for group in groups {
        let mut chars: Vec<char> = group.replace("\n", "").chars().collect();
        chars.sort();
        chars.dedup();
        count = count + (chars.len() as i32);
    }
    return count;
}

fn part_2(input: &str) -> i32 {
    let groups: Vec<&str> = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.trim())
        .collect();
    let mut count: i32 = 0;
    for group in groups {
        let persons = group.split("\n").collect::<Vec<&str>>();

        if persons.len() == 1 {
            let mut chars = group.chars().collect::<Vec<char>>();
            chars.sort();
            chars.dedup();
            count = count + (chars.len() as i32);
            continue;
        }

        let mut chars: HashMap<char, i32> = HashMap::new();
        for person in persons.iter() {
            for c in person.chars() {
                if chars.contains_key(&c) {
                    *chars.get_mut(&c).unwrap() += 1;
                } else {
                    chars.insert(c, 1);
                }
            }
        }
        for (_, co) in chars.into_iter() {
            if co == (persons.len() as i32) {
                count += 1;
            }
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b"
        .to_string();
        let res = part_1(&input);
        assert_eq!(11, res);
    }

    #[test]
    fn test_part_2() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b"
        .to_string();
        let res = part_2(&input);
        assert_eq!(6, res);
    }
}
