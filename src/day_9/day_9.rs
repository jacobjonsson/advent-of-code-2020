#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day9() {
    let input = input_loader::read_input("src/day_9/input.txt");

    let part_1_result = part_1(&input, 25);
    println!("[DAY 9] Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input, 25);
    println!("[DAY 9] Result for part 2: {}", part_2_result);
}

fn part_1(input: &str, preamble_length: i64) -> i64 {
    let mut lower_bound: usize = 0;
    let mut upper_bound: usize = preamble_length as usize;
    let numbers = input
        .trim_end()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for (idx, number) in numbers.iter().enumerate() {
        if idx < preamble_length as usize {
            continue;
        }

        let preamble: &[i64] = &numbers[lower_bound..upper_bound];

        let mut matches = false;
        'outer: for x in preamble {
            for y in preamble {
                if x + y == *number {
                    matches = true;
                    break 'outer;
                }
            }
        }

        if matches {
            lower_bound += 1;
            upper_bound += 1;
            continue;
        } else {
            return *number;
        }
    }

    return -1;
}

fn part_2(input: &str, preamble_length: i64) -> i64 {
    let numbers = input
        .trim_end()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let invalid_number = part_1(input, preamble_length);

    let mut partsum: Vec<i64> = Vec::new();
    partsum.push(0);
    let mut acc = 0;
    for x in &numbers {
        acc += x;
        partsum.push(acc);
    }

    for (idx, _) in partsum.iter().enumerate() {
        let mut j = idx + 2;
        while j < partsum.len() && partsum[j] - partsum[idx] <= invalid_number {
            if partsum[j] - partsum[idx] == invalid_number {
                let slice = &numbers[idx..j];

                let max = match slice.iter().max() {
                    Some(v) => v,
                    None => panic!("Failed to get max"),
                };
                let min = match slice.iter().min() {
                    Some(v) => v,
                    None => panic!("Failed to get min"),
                };

                return min + max;
            } else {
                j += 1;
            }
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
        .to_string();
        assert_eq!(127, part_1(&input, 5));
    }

    #[test]
    fn test_part_2() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
        .to_string();
        assert_eq!(62, part_2(&input, 5));
    }
}
