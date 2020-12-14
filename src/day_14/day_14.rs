use itertools::Itertools;
use std::collections::HashMap;

#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day14() {
    let input = input_loader::read_input("src/day_14/input.txt");

    let part_1_result = part_1(&input);
    println!("[DAY 14] Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("[DAY 14] Result for part 2: {}", part_2_result);
}

fn to_binary(mut decimal: u64) -> String {
    if decimal == 0 {
        let mut bits = String::new();
        for _ in 0..36 {
            bits += "0";
        }
        return bits;
    } else {
        let mut bits = String::new();

        while decimal > 0 {
            if decimal % 2 == 0 {
                bits.push_str("0");
            } else {
                bits.push_str("1");
            }

            decimal /= 2;
        }

        for _ in 0..(36 - bits.len()) {
            bits += "0";
        }

        // reverse the bits
        return bits.chars().rev().collect::<String>();
    }
}

#[derive(Debug)]
struct Instruction {
    memory: u64,
    value: u64,
    binary: String,
}

fn create_instruction(value: &&str) -> Instruction {
    let parts = value.split(" = ").collect::<Vec<&str>>();
    let memory = parts[0].split("[").collect::<Vec<&str>>()[1]
        .split("]")
        .collect::<Vec<&str>>()[0];
    let value = parts[1].parse::<u64>().unwrap();
    return Instruction {
        memory: memory.parse::<u64>().unwrap(),
        value: value,
        binary: to_binary(value),
    };
}

fn load_mask(line: &str) -> String {
    let mask = line.split(" = ").collect::<Vec<&str>>()[1];
    return mask.to_string();
}

fn part_1(input: &str) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut mask = "".to_string();
    for line in input.trim_end().split("\n") {
        if line.contains("mask") {
            mask = load_mask(line);
            continue;
        }

        let instruction = create_instruction(&line);
        let mut new = String::from("");

        for (idx, c) in instruction.binary.chars().enumerate() {
            let mask_char = mask.chars().nth(idx).unwrap();
            if mask_char == 'X' {
                new.push(c);
            } else {
                new.push(mask_char)
            }
        }

        memory.insert(
            instruction.memory,
            isize::from_str_radix(&new, 2).unwrap() as u64,
        );
    }

    let mut total: u64 = 0;
    for (_, v) in memory {
        total += v as u64;
    }
    return total;
}

fn part_2(input: &str) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut mask = "";
    for line in input.trim_end().split("\n") {
        let parts = line.split(" = ").collect::<Vec<&str>>();
        let x = parts[0];
        let y = parts[1];
        if line.contains("mask") {
            mask = y;
            continue;
        }

        let address = x[4..(x.len() - 1)].parse::<u64>().unwrap();
        let value = y.parse::<u64>().unwrap();
        let mut v: u64 = 0;
        let mut extra: Vec<u64> = Vec::new();

        for i in 0..36 {
            let c = mask.chars().nth(mask.len() - 1 - i).unwrap();
            if c == 'X' {
                extra.push(1 << i);
            } else if c == '1' || (address & (1 << i)) > 0 {
                v = v + (1 << i);
            }
        }

        for k in 0..(extra.len() + 1) {
            for t in extra.iter().combinations(k) {
                let s: u64 = t.iter().fold(0, |a, &&b| a + b);
                memory.insert(v + s, value);
            }
        }
    }

    let mut total: u64 = 0;
    for (_, v) in memory {
        total += v as u64;
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> String {
        return "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            .to_string();
    }

    fn get_data_part_2() -> String {
        return "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            .to_string();
    }

    #[test]
    fn test_part_1() {
        assert_eq!(165, part_1(&get_data()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(208, part_2(&get_data_part_2()));
    }
}
