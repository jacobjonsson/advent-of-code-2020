use std::collections::HashMap;

#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day10() {
    let input = input_loader::read_input("src/day_10/input.txt");

    let part_1_result = part_1(&input);
    println!("[DAY 10] Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("[DAY 10] Result for part 2: {}", part_2_result);
}

fn parse_numbers(input: &str) -> Vec<i64> {
    return input
        .trim_end()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
}

fn part_1(input: &str) -> i64 {
    let mut numbers = parse_numbers(&input);
    let max_value = match numbers.iter().max() {
        Some(v) => v + 3,
        None => panic!("Failed to get the max value"),
    };

    numbers.push(max_value);

    numbers.sort();
    let mut prev = 0;
    let mut num_of_1 = 0;
    let mut num_of_3 = 0;
    for number in numbers.iter() {
        match number - prev {
            0 => {
                prev = *number;
                continue;
            }
            1 => {
                num_of_1 += 1;
                prev = *number;
                continue;
            }
            2 => {
                prev = *number;
                continue;
            }
            3 => {
                num_of_3 += 1;
                prev = *number;
                continue;
            }
            _ => {
                println!(
                    "Diff between {} and {} is not 1, 2 or 3. Aborting",
                    number, prev
                );
                break;
            }
        }
    }

    if num_of_1 == 0 || num_of_3 == 0 {
        println!(
            "Either num_of_1: {} or num_of_3: {}, is 0. Aborting",
            num_of_1, num_of_3
        );
        return -1;
    }

    return num_of_1 * num_of_3;
}

fn count_ways(
    numbers: &Vec<i64>,
    start: i64,
    goal: i64,
    memo: &mut HashMap<(usize, i64), i64>,
) -> i64 {
    let memo_key = (numbers.len(), start);
    if memo.contains_key(&memo_key) {
        let value = match memo.get(&memo_key) {
            Some(v) => v,
            None => panic!("Could not get value from memo"),
        };
        return *value;
    }

    let mut ways: i64 = 0;
    if goal - start <= 3 {
        ways += 1;
    }
    if numbers.len() == 0 {
        return ways;
    }
    if numbers[0] - start <= 3 {
        ways += count_ways(&numbers[1..].to_vec(), numbers[0], goal, memo)
    }
    ways += count_ways(&numbers[1..].to_vec(), start, goal, memo);
    memo.insert(memo_key, ways);
    return ways;
}

fn part_2(input: &str) -> i64 {
    let mut numbers = parse_numbers(&input);
    let max_value = match numbers.iter().max() {
        Some(v) => v + 3,
        None => panic!("Failed to get the max value"),
    };

    let mut memo: HashMap<(usize, i64), i64> = HashMap::new();
    numbers.sort();

    return count_ways(&numbers, 0, max_value, &mut memo);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
        .to_string();
        assert_eq!(220, part_1(&input));
    }

    #[test]
    fn test_part_2() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
        .to_string();
        assert_eq!(19208, part_2(&input));
    }
}
