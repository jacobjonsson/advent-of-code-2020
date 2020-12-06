#[path = "../input_loader/input_loader.rs"]
mod input_loader;

fn sum(numbers: &[&i32]) -> i32 {
    return numbers.iter().fold(0, |acc, &x| acc + x);
}

fn product(numbers: &[&i32]) -> i32 {
    return numbers.iter().fold(1, |acc, &x| acc * x);
}

fn parse_int(str: &str) -> i32 {
    return str.parse::<i32>().unwrap();
}

fn part_1(lines: &Vec<&str>) {
    let mut result: i32 = 0;
    'outer: for v in lines.iter() {
        for i in lines.iter() {
            if sum(&[&parse_int(&v), &parse_int(&i)]) == 2020 {
                result = product(&[&parse_int(&v), &parse_int(&i)]);
                break 'outer;
            }
        }
    }

    println!("[DAY 1] Result for part 1: {}", result);
}

fn part_2(lines: &Vec<&str>) {
    let mut result: i32 = 0;
    'outer: for v in lines.iter() {
        for i in lines.iter() {
            for x in lines.iter() {
                if sum(&[&parse_int(&v), &parse_int(&i), &parse_int(&x)]) == 2020 {
                    result = product(&[&parse_int(&v), &parse_int(&i), &parse_int(&x)]);
                    break 'outer;
                }
            }
        }
    }

    println!("[DAY 1] Result for part 2: {}", result);
}

pub fn day1() {
    let input = input_loader::read_input("src/day_1/input.txt");
    let lines = input.trim_end().split("\n").collect::<Vec<&str>>();

    part_1(&lines);
    part_2(&lines);
}
