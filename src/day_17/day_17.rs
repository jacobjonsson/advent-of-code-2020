#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day17() {
    let input = input_loader::read_input("src/day_17/input.txt");

    let part_1_result = part_1(&input);
    println!("[DAY 17] Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("[DAY 17] Result for part 2: {}", part_2_result);
}

fn part_1(_input: &str) -> i32 {
    return 0;
}

fn part_2(_input: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> String {
        return "".to_string();
    }

    #[test]
    fn test_part_1() {
        assert_eq!(0, part_1(&get_data()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(0, part_2(&get_data()));
    }
}
