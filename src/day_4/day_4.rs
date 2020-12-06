#[path = "../input_loader/input_loader.rs"]
mod input_loader;

fn part_2(input: &str) -> i32 {
    let parsed = input.replace("\n\n", "---").replace("\n", " ");
    let groups = parse_groups(&parsed);
    let filtered_groups = groups.iter().filter(|&x| contains_keys(x));

    let mut valid_lines = 0;

    for line in filtered_groups {
        let groups: Vec<&str> = line.split(" ").collect();

        let mut valid = true;
        'inner: for group in groups.iter() {
            let g: Vec<&str> = group.split(":").collect();
            if g[0] == "byr" {
                if g[1].len() != 4 {
                    valid = false;
                    break 'inner;
                }

                let year = g[1].parse::<i32>().unwrap();
                if year < 1920 || year > 2002 {
                    valid = false;
                    break 'inner;
                }

                continue;
            }
            if g[0] == "iyr" {
                if g[1].len() != 4 {
                    valid = false;
                    break 'inner;
                }

                let year = g[1].parse::<i32>().unwrap();
                if year < 2010 || year > 2020 {
                    valid = false;
                    break 'inner;
                }

                continue;
            }
            if g[0] == "eyr" {
                if g[1].len() != 4 {
                    valid = false;
                    break 'inner;
                }

                let year = g[1].parse::<i32>().unwrap();
                if year < 2020 || year > 2030 {
                    valid = false;
                    break 'inner;
                }

                continue;
            }
            if g[0] == "hgt" {
                if !g[1].contains("cm") && !g[1].contains("in") {
                    valid = false;
                    break 'inner;
                }

                if g[1].contains("cm") {
                    let s = g[1].replace("cm", "");
                    let h = s.parse::<i32>().unwrap();
                    if h < 150 || h > 193 {
                        valid = false;
                        break 'inner;
                    }
                }

                if g[1].contains("in") {
                    let s = g[1].replace("in", "");
                    let h = s.parse::<i32>().unwrap();
                    if h < 59 || h > 76 {
                        valid = false;
                        break 'inner;
                    }
                }

                continue;
            }
            if g[0] == "hcl" {
                if !g[1].contains("#") {
                    valid = false;
                    break 'inner;
                }

                let c = g[1].replace("#", "");
                if c.len() != 6 {
                    valid = false;
                    break 'inner;
                }
                continue;
            }
            if g[0] == "ecl" {
                if g[1] != "amb"
                    && g[1] != "blu"
                    && g[1] != "brn"
                    && g[1] != "gry"
                    && g[1] != "grn"
                    && g[1] != "hzl"
                    && g[1] != "oth"
                {
                    valid = false;
                    break 'inner;
                }
                continue;
            }
            if g[0] == "pid" {
                if g[1].len() != 9 {
                    valid = false;
                    break 'inner;
                }

                let ok = g[1].parse::<f64>().is_ok();
                if !ok {
                    valid = false;
                    break 'inner;
                }
                continue;
            }
        }

        if valid {
            valid_lines += 1;
        }
    }

    return valid_lines;
}

pub fn day4() {
    let input = input_loader::read_input("src/day_4/input.txt");
    let part_1_result = part_1(&input);
    let part_2_result = part_2(&input);

    println!("[DAY 4] Result for part 1: {}", part_1_result);
    println!("[DAY 4] Result for part 2: {}", part_2_result);
}

fn contains_keys(line: &str) -> bool {
    return line.contains("byr:")
        && line.contains("iyr:")
        && line.contains("eyr:")
        && line.contains("hgt:")
        && line.contains("hcl:")
        && line.contains("ecl:")
        && line.contains("pid:");
}

fn parse_groups(input: &str) -> Vec<&str> {
    return input.split("---").collect();
}

fn part_1(input: &str) -> i32 {
    let parsed = input.replace("\n\n", "---").replace("\n", " ");
    return parse_groups(&parsed)
        .iter()
        .filter(|&x| contains_keys(x))
        .count() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!(2, part_1(&input))
    }
}
