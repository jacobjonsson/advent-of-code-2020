#[path = "../input_loader/input_loader.rs"]
mod input_loader;

pub fn day13() {
    let input = input_loader::read_input("src/day_13/input.txt");

    let part_1_result = part_1(&input);
    println!("[DAY 13] Result for part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("[DAY 13] Result for part 2: {}", part_2_result);
}

fn part_1(input: &str) -> i32 {
    let parts = input.split("\n").collect::<Vec<&str>>();
    let time = match parts[0].parse::<i32>() {
        Ok(v) => v,
        Err(_) => panic!("Failed to parse time: {}", parts[0]),
    };
    let buses = parts[1]
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|&&x| x != "x")
        .map(|&x| match x.parse::<i32>() {
            Ok(v) => v,
            Err(_) => panic!("Failed to parse {} to int", x),
        })
        .collect::<Vec<i32>>();

    let mut found_bus_id = 0;
    let mut wait_time = 0;
    for idx in time..(time + 50) {
        match buses.iter().find(|&&x| idx % x == 0) {
            Some(v) => {
                found_bus_id = *v;
                wait_time = idx - time;
                break;
            }
            None => continue,
        };
    }

    return found_bus_id * wait_time;
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn crt(values: &Vec<(i64, i64)>) -> Option<i64> {
    let mut n: i64 = 1;
    for (_, num) in values {
        n *= num;
    }

    let mut total = 0;
    for (x, num) in values {
        let p = n / num;
        total += x * mod_inv(p, *num)? * p;
    }

    return Some(total % n);
}

fn part_2(input: &str) -> i64 {
    let parts = input.split("\n").collect::<Vec<&str>>();
    let mut tuples: Vec<(i64, i64)> = Vec::new();
    for (i, num) in parts[1].split(",").enumerate() {
        if num == "x" {
            continue;
        }
        let parsed = num.parse::<i64>().unwrap();
        tuples.push((parsed - i as i64, parsed));
    }

    return match crt(&tuples) {
        Some(v) => v,
        None => panic!("Failed..."),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> String {
        return "939
7,13,x,x,59,x,31,19"
            .to_string();
    }

    #[test]
    fn test_part_1() {
        assert_eq!(295, part_1(&get_data()));
    }

    #[test]
    fn test_part_2_1() {
        assert_eq!(1068781, part_2(&get_data()));
    }

    #[test]
    fn test_part_2_2() {
        assert_eq!(3417, part_2(&"\n17,x,13,19".to_string()));
    }

    #[test]
    fn test_part_2_3() {
        assert_eq!(754018, part_2(&"\n67,7,59,61".to_string()));
    }

    #[test]
    fn test_part_2_4() {
        assert_eq!(779210, part_2(&"\n67,x,7,59,61".to_string()));
    }

    #[test]
    fn test_part_2_5() {
        assert_eq!(1261476, part_2(&"\n67,7,x,59,61".to_string()));
    }

    #[test]
    fn test_part_2_6() {
        assert_eq!(1202161486, part_2(&"\n1789,37,47,1889".to_string()));
    }
}
